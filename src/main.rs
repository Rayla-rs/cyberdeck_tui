use config::Config;
use lazy_static::lazy_static;

use crate::app::App;

pub mod app;
pub mod app_actions;
mod audio_player;
pub mod blt_client;
pub mod config;
pub mod event;
pub mod logging;
mod machine;
pub mod menus;
mod playlist;
mod track;
mod track_widget;
pub mod ui;

pub type Error = Box<dyn std::error::Error>;
pub type AppResult<T> = std::result::Result<T, Error>;

lazy_static! {
    static ref CONFIG: Config = Config::new().unwrap();
}

#[tokio::main]
async fn main() -> AppResult<()> {
    color_eyre::install()?;
    logging::initialize_logging()?;
    trace_dbg!("Logging Initialized");
    let terminal = ratatui::init();
    let result = App::new().await?.run(terminal).await;
    ratatui::restore();
    result
}
