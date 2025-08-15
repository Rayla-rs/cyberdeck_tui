use std::{fmt::Display, path::PathBuf, sync::Arc, time::Duration};

use hhmmss::Hhmmss;
use ratatui::{
    layout::Constraint,
    text::Text,
    widgets::{Cell, Row},
};
use rodio::Source;
use serde::Deserialize;

use crate::{
    app::quick_menu,
    event::AppEvent,
    menus::{Item, LinkedMenu, MenuFrame, TableMenu, TextMenu},
    trace_dbg,
    track::Track,
};

#[derive(Debug, Clone)]
pub struct Playlist {
    pub title: String,
    pub tracks: Vec<Track>,
    pub path: PathBuf,
}

impl Display for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.title))
    }
}

impl Playlist {
    pub fn get_duration(&self) -> Duration {
        self.tracks
            .iter()
            .fold(Duration::default(), |acc, elem| acc + elem.total_duration)
    }
}

impl Item for Playlist {}

impl<'a> Into<Row<'a>> for Playlist {
    fn into(self) -> Row<'a> {
        [
            self.title.clone(),
            self.tracks.len().to_string(),
            self.get_duration().hhmmss(),
        ]
        .iter()
        .map(|elem| Cell::from(Text::from(format!("{elem}"))))
        .collect()
    }
}

pub fn playlist_menu(playlist: Playlist) -> LinkedMenu {
    LinkedMenu::new(Box::new(MenuFrame::new([
        Box::new(
            TableMenu::new(
                playlist.tracks,
                [
                    Constraint::Min(5),
                    Constraint::Length(6),
                    Constraint::Length(8),
                ],
            )
            .with_header(Row::new([
                Cell::new("Title"),
                Cell::new("Artist"),
                Cell::new("Duration"),
            ])),
        ),
        quick_menu(),
    ])))
}

/// Incomplete here for testing only
impl Into<AppEvent> for Playlist {
    fn into(self) -> AppEvent {
        AppEvent::Push(Arc::new(move || playlist_menu(self.clone())))
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
                    // path.push(format!("\\{track}"));
                    path.push(PathBuf::from(track)); // Bug here fix me pls master :3
                    match Track::try_from(path) {
                        Ok(track) => Some(track),
                        Err(err) => {
                            trace_dbg!(err);
                            None
                        }
                    }
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
