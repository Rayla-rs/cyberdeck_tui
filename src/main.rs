use config::Config;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::App;

pub mod app;
mod audio_player;
pub mod blt_menu;
pub mod config;
pub mod event;
mod machine;
mod main_menu;
mod menu;
mod playlist;
mod track;
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
