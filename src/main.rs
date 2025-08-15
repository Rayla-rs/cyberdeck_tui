#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

use config::Config;
use lazy_static::lazy_static;

use crate::app::App;

pub mod app;
mod audio_player;
pub mod config;
pub mod device;
pub mod event;
pub mod fatal;
pub mod logging;
pub mod menus;
mod playlist;
mod track;
pub mod ui;

pub type Error = Box<dyn std::error::Error>;
pub type AppResult<T> = std::result::Result<T, Error>;

lazy_static! {
    static ref CONFIG: Config = Config::new().unwrap();
    static ref USER: String = whoami::username();
    static ref DEVICE: String = whoami::devicename();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    logging::initialize_logging()?;
    trace_dbg!("Logging Initialized");
    let mut terminal = ratatui::init();
    if let Err(report) = App::new().await.run(&mut terminal).await {
        crate::fatal::FatalWidget(report).run(&mut terminal).await?;
    }
    // check env var for reboot on exit
    ratatui::restore();
    Ok(())
}

// TODO -> dyn Options updated by app_state
