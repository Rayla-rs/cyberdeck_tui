use std::fmt::Debug;

use hhmmss::Hhmmss;
use rodio::{OutputStream, Sink};
use tracing::{Level, instrument, span, trace};

use crate::track::Track;

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

    pub fn queue_playlist(&mut self, tracks: Vec<Track>) {
        let span = span!(Level::TRACE, "queue playlist");
        let guard = span.enter();

        for track in tracks.into_iter().rev() {
            self.push_track(track);
        }

        drop(guard);
    }

    pub fn tick(&mut self) -> color_eyre::Result<()> {
        if !self.sink.is_paused() {
            match self.current.as_ref() {
                Some(current) => {
                    if self.sink.empty() && self.has_next() {
                        self.history.push(current.clone());
                        self.play()?;
                    }
                }
                None => {
                    self.play()?;
                }
            }
        }
        Ok(())
    }

    pub fn play(&mut self) -> color_eyre::Result<()> {
        self.next()?;
        self.resume();
        Ok(())
    }

    pub fn resume(&mut self) {
        self.sink.play();
    }

    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }

    fn next(&mut self) -> color_eyre::Result<()> {
        self.current = self.queue.pop();
        self.sink.clear();
        if let Some(track) = self.current.as_ref() {
            trace!("play_next");
            self.sink.append(track.decode()?);
        }
        Ok(())
    }

    #[instrument]
    pub fn push_track(&mut self, track: Track) {
        trace!("pushed track {:?}", track);
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
            Some(current) => self
                .sink
                .get_pos()
                .div_duration_f64(current.total_duration)
                .clamp(0.0, 1.0),
            None => 0.0,
        }
    }

    pub fn get_progress_label(&self) -> String {
        format!(
            "{}|{}",
            self.sink.get_pos().hhmmss(),
            match self.current.as_ref() {
                Some(current) => {
                    current.total_duration.hhmmss()
                }
                None => String::from("NAN"),
            }
        )
    }

    pub fn empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
}

impl<'a> AudioPlayer {
    pub fn get_current(&'a self) -> Option<&'a Track> {
        self.current.as_ref()
    }
}
