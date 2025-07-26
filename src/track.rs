use audiotags::{AudioTag, Tag};
use rodio::{Decoder, Source};

use std::{
    fs::{File, OpenOptions},
    io::BufReader,
    path::PathBuf,
    time::Duration,
};

pub struct Track {
    path: PathBuf,
    tags: Box<dyn AudioTag>, // Refactor to make less err prone
    decoder: Decoder<BufReader<File>>,
}

impl TryFrom<PathBuf> for Track {
    type Error = crate::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            path: value.clone(),
            tags: Tag::new().read_from_path(value.clone())?,
            decoder: Decoder::try_from(OpenOptions::new().open(value)?)?,
        })
    }
}

impl Track {
    pub fn total_duration(&self) -> Option<Duration> {
        self.decoder.total_duration()
    }
    pub fn get_extension(&self) {}

    pub fn data(&self) -> [String; 3] {
        [
            self.tags.title().unwrap_or_default().to_string(),
            self.tags.artist().unwrap_or_default().to_string(),
            chrono::Duration::seconds(self.tags.duration().unwrap_or_default() as i64).to_string(),
        ]
    }
}
