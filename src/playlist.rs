use std::path::PathBuf;

use ratatui::{
    text::Text,
    widgets::{Cell, Row},
};
use serde::Deserialize;

use crate::track::Track;

#[derive(Clone)]
pub struct Playlist {
    pub title: String,
    pub tracks: Vec<Track>,
    pub path: PathBuf,
}

impl Playlist {
    pub fn data(&self) -> [String; 3] {
        [
            self.title.clone(),
            self.tracks.len().to_string(),
            String::from("DURATION"),
        ]
    }
}

impl<'a> Into<Row<'a>> for &'a Playlist {
    fn into(self) -> Row<'a> {
        self.data()
            .iter()
            .map(|elem| Cell::from(Text::from(format!("{elem}"))))
            .collect()
    }
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
                    let _ = path.pop();
                    path.push(format!("\\{track}"));
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
