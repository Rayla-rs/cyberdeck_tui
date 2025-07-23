use std::{fs::File, sync::Arc, time::Duration};

use rodio::{Decoder, OutputStream, Sink, Source, source::SineWave};

use crate::AppResult;

pub struct AudioPlayer {
    stream_handle: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        Self {
            stream_handle,
            sink,
        }
    }

    pub fn enqueue(&mut self, track: File) -> AppResult<()> {
        let decoder = Decoder::try_from(track)?;
        self.sink.append(decoder);
        Ok(())
    }

    pub fn play<T: Source>(&mut self, source: File) {}

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

    pub fn test(&mut self) {
        self.sink.append(
            SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.25))
                .amplify(0.20),
        );
    }
}
