use color_eyre::eyre::OptionExt;
use serde::Deserialize;

use crate::playlist::Playlist;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub music_dir: String,
}

impl Config {
    /// Create new config from users config!
    pub fn new() -> color_eyre::Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string(
            dirs::config_dir()
                .ok_or_eyre("User config directory not found!")?
                .join("cyberdeck_tui")
                .join("config.toml"),
        )?)?)
    }

    pub fn load_playlists(&self) -> impl Iterator<Item = Playlist> {
        std::fs::read_dir(self.music_dir.clone())
            .into_iter()
            .flat_map(|read_dir| {
                read_dir.filter_map(|entry| Some(Playlist::try_from(entry.ok()?.path()).ok()?))
            })
    }
}
