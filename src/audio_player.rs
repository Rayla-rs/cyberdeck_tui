use std::{fs::File, time::Duration};

use rodio::{Decoder, OutputStream, Sink, Source, source::SineWave};

use crate::{AppResult, track::Track};
use tokio::sync::mpsc;

enum Event {
    Action(AudioAction),
}

enum AudioAction {
    Play,
    Pause,
    Next,
    Previouse,
    PushBack(Track),
    PushFront(Track),
}

struct AudioEventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
}
impl AudioEventHandler {
    /// Constructs a new instance of [`EventHandler`] and spawns a new thread to handle events.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        // let actor = EventTask::new(sender.clone());
        // tokio::spawn(async { actor.run().await });
        Self { sender, receiver }
    }
}

pub struct AudioPlayer {
    stream_handle: OutputStream,
    sink: Sink,
    // History
    // Future (queue)
    current: Option<Track>,
    queue: Vec<Track>,
    history: Vec<Track>,
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

    pub fn stuff(&self) {}

    // TODO tick
    //
    //
    pub fn enqueue(&mut self, track: File) -> AppResult<()> {
        let decoder = Decoder::try_from(track)?;
        self.sink.append(decoder);
        // self.sink.append(EmptyCallback::new(Box::new(|| {})));
        Ok(())
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

    pub fn test(&mut self) {
        self.sink.append(
            SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.25))
                .amplify(0.20),
        );
    }
}
