use std::path::PathBuf;
use clap::Parser;
use std::io;
use tracing::{error, info};
use tracing_subscriber::fmt::time::ChronoLocal;

#[derive(Parser)]
pub struct App {
    #[arg(short, long)]
    pub assets: PathBuf
}

/// This function will check if the arguments passed to the program are valid
fn check_app_args(app: &App) -> io::Result<()> {
    if !app.assets.exists() || !std::fs::metadata(&app.assets)?.is_dir() {
        error!(target: "AssetsPreCheck", "The given assets folder path is invalid");
        std::process::exit(1);
    }

    let config_path = app.assets.join("config.toml");
    if !config_path.exists() || !std::fs::metadata(&config_path)?.is_file() {
        error!(target: "AssetsPreCheck", "Cannot find the required `config.toml` file in the assets folder");
        std::process::exit(1);
    }

    let token_path = app.assets.join("token");
    if !token_path.exists() || !std::fs::metadata(&token_path)?.is_file() {
        error!(target: "AssetsPreCheck", "Cannot find the required `token` file in the 'assets' folder");
        std::process::exit(1);
    }



    info!(
        target: "App",
        "Config located at {}",
        config_path.canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    );
    info!(
        target: "App",
        "Token  located at {}",
        token_path.canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    // init tracing_subscriber
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_timer(ChronoLocal::default())
        .init();

    let app = App::parse();

    if let Err(e) = check_app_args(&app) {
        error!(target: "App", "Cannot init the app: {e}");
        std::process::exit(1);
    };
}
