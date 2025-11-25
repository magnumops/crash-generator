use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use std::path::Path;

#[derive(Parser)]
#[command(name = "magnum")]
#[command(about = "The Logos Adversarial Testing Suite", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Crash Generator (Fake Exchange)
    Crash {
        /// Port to run the proxy on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Analyze a liquidation event (The Pathologist)
    Analyze {
        /// Path to the evidence CSV file
        #[arg(short, long)]
        file: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Определяем путь к виртуальному окружению
    // Предполагаем, что бинарник запускается из корня crash-generator
    let venv_python = "./venv/bin/python3";
    
    if !Path::new(venv_python).exists() {
        eprintln!("CRITICAL ERROR: venv not found at {}. Please run 'python3 -m venv venv' first.", venv_python);
        std::process::exit(1);
    }

    match cli.command {
        Commands::Crash { port } => {
            println!("[MAGNUM] initializing CRASH GENERATOR on port {}...", port);
            println!("[MAGNUM] Target: localhost:{}", port);
            
            let mut child = Command::new(venv_python)
                .args(&["-m", "logos.proxies.binance_proxy"])
                .env("PORT", port.to_string())
                .current_dir("./vendor/logos") // Запускаем из контекста подмодуля
                .env("PYTHONPATH", ".")        // Чтобы Python видел пакет logos
                .spawn()?;

            child.wait()?;
        }
        Commands::Analyze { file } => {
            println!("[MAGNUM] initializing PATHOLOGIST...");
            println!("[MAGNUM] Examining evidence: {}", file);
            
            // Нам нужно передать абсолютный путь к файлу, так как мы меняем workdir
            let abs_path = std::fs::canonicalize(&file).unwrap_or_else(|_| Path::new(&file).to_path_buf());
            
            let status = Command::new(venv_python)
                .args(&["-m", "logos.forensic_delegator"])
                .current_dir("./vendor/logos")
                .env("PYTHONPATH", ".")
                .arg(format!("csv_path={}", abs_path.display())) # Передадим аргументом (нужно доработать python, но пока так)
                .status()?;
                
            if !status.success() {
                eprintln!("[MAGNUM] Investigation failed.");
            }
        }
    }

    Ok(())
}
