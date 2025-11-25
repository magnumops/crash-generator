use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;
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

    // ПОПРАВКА: Строим абсолютный путь к venv/bin/python3 БЕЗ canonicalize.
    // canonicalize убивает контекст venv, превращая путь в системный /usr/bin/python
    let cwd = std::env::current_dir()?;
    let python_path = cwd.join("venv").join("bin").join("python3");

    if !python_path.exists() {
        eprintln!("CRITICAL ERROR: venv python not found at {:?}", python_path);
        std::process::exit(1);
    }

    println!("[MAGNUM] Using Python Interpreter: {:?}", python_path);

    match cli.command {
        Commands::Crash { port } => {
            println!("[MAGNUM] Target: localhost:{}", port);
            
            let mut child = Command::new(&python_path)
                .args(&["-m", "logos.proxies.binance_proxy"])
                .env("PORT", port.to_string())
                .current_dir("./vendor/logos") 
                .env("PYTHONPATH", ".")
                .spawn()?;

            child.wait()?;
        }
        Commands::Analyze { file } => {
            // Путь к файлу тоже делаем абсолютным
            let file_path = Path::new(&file);
            let abs_file_path = if file_path.is_absolute() {
                file_path.to_path_buf()
            } else {
                cwd.join(file_path)
            };
            
            println!("[MAGNUM] Examining evidence: {:?}", abs_file_path);

            let status = Command::new(&python_path)
                .args(&["-m", "logos.forensic_delegator", abs_file_path.to_str().unwrap()])
                .current_dir("./vendor/logos")
                .env("PYTHONPATH", ".")
                .status()?;
                
            if !status.success() {
                eprintln!("[MAGNUM] Investigation failed.");
            }
        }
    }

    Ok(())
}
