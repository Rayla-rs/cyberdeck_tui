use audiotags::{AudioTag, Tag};
use hhmmss::Hhmmss;
use rodio::{Decoder, Source};

use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::BufReader,
    path::PathBuf,
    time::Duration,
};

use crate::trace_dbg;

pub struct Track {
    path: PathBuf,
    tags: Box<dyn AudioTag>, // Refactor to make less err prone
    decoder: Decoder<BufReader<File>>,
}

impl Debug for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Track_DBG_TODO")
    }
}

impl Clone for Track {
    fn clone(&self) -> Self {
        // Expect it to work because &self exists
        Self::try_from(self.path.clone()).unwrap()
    }
}

impl TryFrom<PathBuf> for Track {
    type Error = crate::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            path: value.clone(),
            tags: Tag::new().read_from_path(value.clone())?,
            decoder: Decoder::try_from(OpenOptions::new().read(true).open(value)?)?,
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
            Duration::from_secs(self.tags.duration().unwrap_or_default() as u64).hhmmss(),
        ]
    }
}
