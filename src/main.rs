mod tui;

use color_eyre::Result;
use env_logger::Env;
use log::*;
use tui::Tui;

async fn tokio_main() -> Result<()> {
    init_logging();
    let tui = Tui::new()?;
    tui.enter()?;
    tui.exit()?;
    Ok(())
}

fn init_logging() {
    let log_env = Env::default().filter_or("debug", "debug");
    env_logger::init_from_env(log_env);
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
