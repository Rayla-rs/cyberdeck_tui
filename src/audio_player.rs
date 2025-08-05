use std::fmt::Debug;

use chrono::Duration;
use hhmmss::Hhmmss;
use rodio::{Decoder, OutputStream, Sink, Source, source::SineWave};
use tracing::{Level, Span, info, instrument, span, trace};

use crate::{AppResult, playlist::Playlist, track::Track};

pub struct AudioPlayer {
    stream_handle: OutputStream,
    sink: Sink,
    current: Option<Track>,
    queue: Vec<Track>,
    history: Vec<Track>,
}

impl Debug for AudioPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "AudioPlayer{{current: {:?}, queue: {:?}, history: {:?}}}",
            self.current, self.queue, self.history
        ))
    }
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        Self {
            stream_handle,
            sink,
            current: None,
            queue: vec![],
            history: vec![],
        }
    }

    pub fn queue_playlist(&mut self, playlist: Playlist) {
        let span = span!(Level::TRACE, "queue playlist");
        let guard = span.enter();

        for track in playlist.tracks.into_iter().rev() {
            self.push_track(track);
        }

        drop(guard);
    }

    pub fn tick(&mut self) {
        // change so that we move current
        if !self.sink.is_paused() {
            match self.current.as_ref() {
                Some(current) => {
                    if self.sink.empty() && self.has_next() {
                        self.history.push(current.clone());
                        self.play();
                    }
                }
                None => {
                    self.play();
                }
            }
        }
    }

    pub fn play(&mut self) {
        self.next();
        self.sink.play();
    }

    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }

    fn next(&mut self) {
        self.current = self.queue.pop();
        self.sink.clear();
        // Eww clone pls fix someday pls
        if let Some(track) = self.current.clone() {
            trace!("play_next");
            self.sink.append(track.decoder);
        }
    }

    #[instrument]
    pub fn push_track(&mut self, track: Track) {
        trace!("pushed track {}", track);
        self.queue.push(track);
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn skip(&mut self) {
        self.sink.skip_one();
    }

    pub fn restart(&mut self) {
        // TODO
        // seek beginning
    }

    pub fn get_progress(&self) -> f64 {
        match self.current.as_ref() {
            Some(current) => match current.total_duration() {
                Some(total_duration) => self
                    .sink
                    .get_pos()
                    .div_duration_f64(total_duration)
                    .clamp(0.0, 1.0),
                None => 0.0,
            },
            None => 0.0,
        }
    }

    pub fn get_progress_label(&self) -> String {
        format!(
            "{}|{}",
            self.sink.get_pos().hhmmss(),
            match self.current.as_ref() {
                Some(current) => {
                    match current.total_duration() {
                        Some(total_duration) => total_duration.hhmmss(),
                        None => String::from("NAN"),
                    }
                }
                None => String::from("NAN"),
            }
        )
    }
}

impl<'a> AudioPlayer {
    pub fn get_current(&'a self) -> Option<&'a Track> {
        self.current.as_ref()
    }
}
