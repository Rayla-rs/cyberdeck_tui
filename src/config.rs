use serde::Deserialize;

use crate::AppResult;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub music_dir: String,
}

impl Config {
    /// Create new config from users config!
    pub fn new() -> AppResult<Self> {
        // let mut path = dirs::config_dir()
        //     .ok_or("User config directory not found!")?
        //     .join("cyberdeck_tui");
        // if !std::fs::exists(path.clone())? {
        //     std::fs::create_dir(path);
        // }

        Ok(toml::from_str(&std::fs::read_to_string(
            dirs::config_dir()
                .ok_or("User config directory not found!")?
                .join("cyberdeck_tui")
                .join("config.toml"),
        )?)?)
    }

    fn get_music_dir(&self) {
        std::fs::read_dir(self.music_dir.clone())
            .unwrap()
            .for_each(|elem| {
                elem.unwrap().file_type().unwrap().is_dir();
            });
    }
}
