use config::Config;

use crate::app::App;

pub mod app;
mod audio_player;
pub mod config;
pub mod event;
mod machine;
pub mod menus;
mod playlist;
mod track;
mod track_widget;
pub mod ui;

pub type Error = Box<dyn std::error::Error>;
pub type AppResult<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> AppResult<()> {
    color_eyre::install()?;
    let config = Config::new()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await?;
    ratatui::restore();
    Ok(())
}

// TODO make "corupted" text using rng for fun when bad meta data
