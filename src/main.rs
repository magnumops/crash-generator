use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

#[derive(Parser)]
#[command(name = "magnum")]
#[command(about = "The Logos Adversarial Testing Suite", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Crash {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    Analyze {
        #[arg(short, long)]
        file: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cwd = env::current_dir()?;

    // ЛОГИКА ВЫБОРА PYTHON:
    // 1. Сначала смотрим переменную окружения (для Docker)
    // 2. Если нет, ищем локальный venv (для разработки)
    let python_path = if let Ok(env_path) = env::var("MAGNUM_PYTHON_EXEC") {
        Path::new(&env_path).to_path_buf()
    } else {
        cwd.join("venv").join("bin").join("python3")
    };

    if !python_path.exists() {
        eprintln!("CRITICAL ERROR: Python interpreter not found at {:?}", python_path);
        eprintln!("Hint: In Docker, set MAGNUM_PYTHON_EXEC. Locally, check your venv.");
        std::process::exit(1);
    }

    println!("[MAGNUM] Using Python Interpreter: {:?}", python_path);

    // Определяем корень Python-кода (logos)
    // В Docker мы положим его в /app/vendor/logos, локально он в ./vendor/logos
    let python_root = if env::var("MAGNUM_IN_DOCKER").is_ok() {
        Path::new("/app/vendor/logos").to_path_buf()
    } else {
        cwd.join("vendor").join("logos")
    };

    match cli.command {
        Commands::Crash { port } => {
            println!("[MAGNUM] Target: 0.0.0.0:{}", port);
            
            let mut child = Command::new(&python_path)
                .args(&["-m", "logos.proxies.binance_proxy"])
                .env("PORT", port.to_string())
                .current_dir(&python_root) 
                .env("PYTHONPATH", ".")
                .spawn()?;

            child.wait()?;
        }
        Commands::Analyze { file } => {
            let file_path = Path::new(&file);
            let abs_file_path = if file_path.is_absolute() {
                file_path.to_path_buf()
            } else {
                cwd.join(file_path)
            };
            
            println!("[MAGNUM] Examining evidence: {:?}", abs_file_path);

            let status = Command::new(&python_path)
                .args(&["-m", "logos.forensic_delegator", abs_file_path.to_str().unwrap()])
                .current_dir(&python_root)
                .env("PYTHONPATH", ".")
                .status()?;
                
            if !status.success() {
                eprintln!("[MAGNUM] Investigation failed.");
            }
        }
    }

    Ok(())
}
