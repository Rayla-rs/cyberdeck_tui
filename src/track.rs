use audiotags::Tag;
use color_eyre::eyre::Context;
use hhmmss::Hhmmss;
use ratatui::{
    text::Text,
    widgets::{Cell, Row},
};
// use hhmmss::Hhmmss;
use rodio::Decoder;

use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::BufReader,
    path::PathBuf,
    time::Duration,
};

use crate::{event::AppEvent, menus::Item};

// make into decoder for track and on cp do not rebuild

#[derive(Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
    pub total_duration: Duration,
}

impl Track {
    pub fn decode(&self) -> color_eyre::Result<Decoder<BufReader<File>>> {
        Decoder::try_from(
            OpenOptions::new()
                .read(true)
                .open(self.path.clone())
                .wrap_err("Failed to open file!")?,
        )
        .wrap_err("Rodio decoder err!")
    }
}

impl Debug for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Track_DBG_TODO")
    }
}

impl TryFrom<PathBuf> for Track {
    type Error = crate::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let tag = Tag::new().read_from_path(value.clone())?;
        Ok(Self {
            path: value.clone(),
            title: tag.title().unwrap_or_default().to_string(),
            artist: tag.artist().unwrap_or_default().to_string(),
            total_duration: match tag.duration() {
                Some(dur) => Duration::from_secs_f64(dur),
                None => mp3_duration::from_path(value).unwrap_or_default(),
            },
        })
    }
}

impl Into<AppEvent> for Track {
    fn into(self) -> AppEvent {
        AppEvent::Play(vec![self])
    }
}

impl<'a> Into<Row<'a>> for Track {
    fn into(self) -> Row<'a> {
        [
            self.title.clone(),
            self.artist.clone(),
            self.total_duration.hhmmss(),
        ]
        .iter()
        .map(|elem| Cell::from(Text::from(format!("{elem}"))))
        .collect()
    }
}

impl Item for Track {}
