use clap::{Parser, Subcommand};
use anyhow::Result;
use ogl_core::{detect_installation, config_manager::ConfigManager, engine_manager::EngineManager};
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "ogl-cli")]
#[command(about = "OpenGothicLauncher CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect existing Gothic installations
    Detect,
    /// List installed OpenGothic engines
    Engines,
    /// List detected mods
    Mods,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let cfg_manager = ConfigManager::new()?;
    let engine_manager = EngineManager::new()?;

    match &cli.command {
        Commands::Detect => {
            info!("Running detection...");
            match detect_installation() {
                Ok(install) => {
                    info!("Detected Gothic Installation at: {:?}", install.root_path);
                    let mut config = cfg_manager.load()?;
                    config.gothic_path = Some(install.root_path.clone());
                    cfg_manager.save(&config)?;
                    info!("Saved path to configuration.");
                }
                Err(e) => {
                    error!("Failed to detect Gothic: {}", e);
                }
            }
        },
        Commands::Engines => {
            let installed = engine_manager.list_installed()?;
            if installed.is_empty() {
                info!("No OpenGothic engines installed.");
            } else {
                info!("Installed OpenGothic engines:");
                for e in installed {
                    info!("  - Version: {}", e.version);
                }
            }
        },
        Commands::Mods => {
            let config = cfg_manager.load()?;
            if let Some(path) = config.gothic_path {
                let mod_manager = ogl_mods::ModManager::new(path);
                match mod_manager.scan_mods() {
                    Ok(mods) => {
                        info!("Detected mods:");
                        for m in mods {
                            info!("  - {} (VDF: {})", m.name, m.is_vdf);
                        }
                    }
                    Err(e) => error!("Failed to scan mods: {}", e),
                }
            } else {
                error!("No Gothic path configured. Run `detect` first.");
            }
        }
    }

    Ok(())
}
