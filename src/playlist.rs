use std::path::PathBuf;

use serde::Deserialize;

use crate::track::Track;

pub struct Playlist {
    pub title: String,
    pub tracks: Vec<Track>,
    pub path: PathBuf,
}

impl TryFrom<PathBuf> for Playlist {
    type Error = crate::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(PlyData::try_from(value)?)
    }
}

impl TryFrom<PlyData> for Playlist {
    type Error = crate::Error;
    fn try_from(value: PlyData) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            tracks: value
                .tracks
                .iter()
                .filter_map(|track| {
                    let mut path = value.path.clone();
                    path.push(track);
                    Track::try_from(path).ok()
                })
                .collect::<Vec<_>>(),
            path: value.path,
        })
    }
}

/// Playlist data file struct for deserializing
#[derive(Deserialize, Debug)]
struct PlyData {
    #[serde(default)]
    title: String,
    #[serde(default)]
    tracks: Vec<String>,
    #[serde(skip)]
    path: PathBuf,
}

impl TryFrom<PathBuf> for PlyData {
    type Error = crate::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            path: value.clone(),
            ..toml::from_str(&std::fs::read_to_string(value)?)?
        })
    }
}
