use color_eyre::eyre::OptionExt;
use serde::Deserialize;

use crate::playlist::Playlist;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub music_dir: String,
    #[serde(skip)]
    pub playlists: Vec<Playlist>,
}

impl Config {
    /// Create new config from users config!
    pub fn new() -> color_eyre::Result<Self> {
        Ok(toml::from_str::<Self>(&std::fs::read_to_string(
            dirs::config_dir()
                .ok_or_eyre("User config directory not found!")?
                .join("cyberdeck_tui")
                .join("config.toml"),
        )?)?
        .load_playlists())
    }

    fn load_playlists(mut self) -> Self {
        self.playlists = std::fs::read_dir(self.music_dir.clone())
            .into_iter()
            .flat_map(|read_dir| {
                read_dir.filter_map(|entry| {
                    Some(
                        Playlist::try_from(
                            entry
                                .ok()
                                .filter(|read_dir| {
                                    read_dir
                                        .path()
                                        .extension()
                                        .is_some_and(|extension| extension == "toml")
                                })?
                                .path(),
                        )
                        .ok()?,
                    )
                })
            })
            .collect();
        self
    }
}
