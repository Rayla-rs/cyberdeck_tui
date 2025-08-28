use bluer::{Adapter, AdapterEvent, Address, DiscoveryFilter, Session};
use color_eyre::eyre::OptionExt;
use futures::{
    FutureExt, StreamExt, pin_mut,
    stream::{LocalBoxStream, Skip},
};
use ratatui::{
    crossterm::event::Event as CrosstermEvent,
    widgets::{Cell, Row},
};
use std::{fmt::Debug, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tracing::trace;

use crate::{
    device::Device,
    menus::{Item, LinkedMenu},
    track::Track,
};

/// The frequency at which tick events are emitted.
const TICK_FPS: f64 = 15.0;

/// Representation of all possible events.
#[derive(Clone, Debug)]
pub enum Event {
    /// An event that is emitted on a regular schedule.
    ///
    /// Use this event to run any code which has to run outside of being a direct response to a user
    /// event. e.g. polling exernal systems, updating animations, or rendering the UI based on a
    /// fixed frame rate.
    Tick,
    /// Crossterm events.
    ///
    /// These events are emitted by the terminal.
    Crossterm(CrosstermEvent),
    /// Application events.
    ///
    /// Use this event to emit custom events that are specific to your application.
    App(AppEvent),
    Blt(BltEvent),
}

/// Application events.
#[derive(Clone)]
pub enum AppEvent {
    /// Move up
    Up,
    /// Move down
    Down,
    /// Submit
    Enter,
    /// Quit the application.
    Quit,
    /// Remove leaf linked menu
    Pop,
    /// Add leaf linked menu
    Push(Arc<dyn Fn() -> LinkedMenu + Send + Sync>),
    /// Play a playlist
    Play(Vec<Track>),
    /// Resume track
    Resume,
    /// Pause track
    Pause,
    /// Skip track
    Skip,
    /// Connect with Device
    Connect(Device),
    /// Disconect Device
    Disconnect(Device),
    /// Trust Device
    Trust(Device),
    /// Untrust Device
    Untrust(Device),

    Debug,
}

impl Debug for AppEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "AppEvent::{}",
            match self {
                Self::Up => String::from("Up"),
                Self::Down => String::from("Down"),
                Self::Enter => String::from("Enter"),
                Self::Quit => String::from("Quit"),
                Self::Pop => String::from("Pop"),
                Self::Push(_) => String::from("Push(..)"),
                Self::Play(_) => String::from("Play(..)"),
                Self::Resume => String::from("Resume"),
                Self::Pause => String::from("Pause"),
                Self::Skip => String::from("Skip"),
                Self::Connect(device) => format!("Connect({})", device.address.to_string()),
                Self::Trust(device) => format!("Trust({})", device.address.to_string()),
                Self::Untrust(device) => format!("Untrust({})", device.address.to_string()),
                Self::Disconnect(device) => format!("Disconnect({})", device.address.to_string()),
                Self::Debug => String::from("Debug"),
            }
        ))
    }
}

impl Into<Row<'static>> for AppEvent {
    fn into(self) -> Row<'static> {
        Row::new([Cell::new(match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Enter => "Enter",
            Self::Quit => "Quit",
            Self::Pop => "Pop",
            Self::Push(_) => "Push",
            Self::Play(_) => "Play",
            Self::Resume => "Resume",
            Self::Pause => "Pause",
            Self::Skip => "Skip",
            Self::Connect(_) => "Connect",
            Self::Trust(_) => "Trust",
            Self::Untrust(_) => "Untrust",
            Self::Disconnect(_) => "Disconnect",
            Self::Debug => "Debug",
        })])
    }
}

impl Item for AppEvent {}

#[derive(Clone, Debug)]
pub enum BltEvent {
    Add(Device),
    Remove(Address),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`] and spawns a new thread to handle events.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = EventTask::new(sender.clone());
        tokio::spawn(async { actor.run().await });
        Self { sender, receiver }
    }

    /// Receives an event from the sender.
    ///
    /// This function blocks until an event is received.
    ///
    /// # Errors
    ///
    /// This function returns an error if the sender channel is disconnected. This can happen if an
    /// error occurs in the event thread. In practice, this should not happen unless there is a
    /// problem with the underlying terminal.
    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
    }

    /// Queue an app event to be sent to the event receiver.
    ///
    /// This is useful for sending events to the event handler which will be processed by the next
    /// iteration of the application's event loop.
    pub fn send(&mut self, app_event: AppEvent) {
        // Ignore the result as the reciever cannot be dropped while this struct still has a
        // reference to it
        let _ = self.sender.send(Event::App(app_event));
    }
}

/// A thread that handles reading crossterm events and emitting tick events on a regular schedule.
struct EventTask {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
}

impl EventTask {
    /// Constructs a new instance of [`EventThread`].
    fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    /// Runs the event thread.
    ///
    /// This function emits tick events at a fixed rate and polls for crossterm events in between.
    async fn run(self) -> color_eyre::Result<()> {
        let service = BltService::new().await?;

        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next().fuse();

            tokio::select! {
              _ = self.sender.closed() => {
                  break;
              }
              _ = tick_delay => {
                  self.send(Event::Tick);
              }
              Some(Ok(evt)) = crossterm_event => {
                  self.send(Event::Crossterm(evt));
              }
              Some(device_event) = device_event => {

              }
            };
        }
        Ok(())
    }

    /// Sends an event to the receiver.
    fn send(&self, event: Event) {
        // Ignores the result because shutting down the app drops the receiver, which causes the send
        // operation to fail. This is expected behavior and should not panic.
        let _ = self.sender.send(event);
    }

    fn handle_blt_event(&mut self, event: AdapterEvent) {
        match event {
            AdapterEvent::DeviceAdded(address) => {
                match Device::new(&adapter, address).await {
                    Ok(device) => self.send(Event::Blt(BltEvent::Add(device))),
                    Err(err) => {
                        trace!("Err: {}", err);
                    }
                };
            }
            AdapterEvent::DeviceRemoved(address) => {
                self.send(Event::Blt(BltEvent::Remove(address)));
            }
            _ => {
                // device updated
            }
        };
    }
}

struct BltService {
    adapter: Adapter,
    stream: Option<LocalBoxStream<'static, AdapterEvent>>,
}

impl BltService {
    async fn new() -> color_eyre::Result<Self> {
        let session = Session::new().await?;
        let adapter = session.default_adapter().await?;

        // Discovery filter
        let filter = DiscoveryFilter {
            transport: bluer::DiscoveryTransport::Auto,
            ..DiscoveryFilter::default()
        };
        adapter.set_discovery_filter(filter).await?;

        Ok(Self {
            adapter,
            stream: None,
        })
    }

    pub async fn set_state(&mut self, enable: bool) -> color_eyre::Result<()> {
        let active = self.stream.is_none();

        match (enable, active) {
            (true, false) => {
                self.stream = Some(self.adapter.discover_devices().await?.boxed_local())
            }
            (false, true) => {
                self.stream = None;
            }
            _ => {}
        }
        Ok(())
    }

    async fn next(&mut self) -> () {
        match self.stream {
            Some(stream) => stream.next().fuse(),
            None => (async || None)().fuse(),
        }
    }
}
