use std::collections::HashMap;
use std::ops::Not;

use crate::device::Device;
use crate::event::BltEvent;
use crate::menus::{self, LinkedMenu, Menu, TableMenu};
use crate::trace_dbg;
use crate::{
    audio_player::AudioPlayer,
    event::{AppEvent, Event, EventHandler},
};
use bluer::Address;
use bluetui::app::AppResult;
use ratatui::layout::Constraint;
use ratatui::widgets::{Cell, Row};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

pub struct AppState {
    pub player: AudioPlayer,
    pub devices: HashMap<Address, Device>,
}

impl AppState {
    fn new() -> Self {
        Self {
            player: AudioPlayer::new(),
            devices: HashMap::default(),
        }
    }

    pub fn cloned_devices(&self) -> Vec<Device> {
        self.devices.clone().into_values().collect()
    }
}

/// Application.
pub struct App {
    pub state: AppState,
    pub menu: LinkedMenu,
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Self {
        Self {
            state: AppState::new(),
            running: true,
            events: EventHandler::new(),
            menu: menus::make_test_menu(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick()?,
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Up => self.up(),
                    AppEvent::Down => self.down(),
                    AppEvent::Enter => {
                        self.enter()?;
                    }
                    AppEvent::Quit => self.quit(),
                    AppEvent::Pop => {
                        self.menu.pop();
                    }
                    AppEvent::Push(func) => {
                        self.menu.push(func());
                    }
                    AppEvent::Play(playlist) => {
                        self.state.player.queue_playlist(playlist);
                    }
                    AppEvent::Resume => {
                        self.state.player.resume();
                    }
                    AppEvent::Pause => {
                        self.state.player.pause();
                    }
                    AppEvent::Debug => {
                        trace_dbg!("Debuged");
                    }
                },
                Event::Blt(device_event) => match device_event {
                    BltEvent::Add(dev) => {
                        self.add_device(dev);
                    }
                    BltEvent::Remove(addr) => self.remove_device(addr),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Up => self.events.send(AppEvent::Up),
            KeyCode::Down => self.events.send(AppEvent::Down),
            KeyCode::Enter => self.events.send(AppEvent::Enter),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) -> color_eyre::Result<()> {
        self.state.player.tick()?;
        self.menu.tick(&self.state)
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn enter(&mut self) -> color_eyre::Result<()> {
        Ok(if let Some(event) = self.menu.enter()? {
            self.events.send(event);
        })
    }

    pub fn up(&mut self) {
        self.menu.up();
    }

    pub fn down(&mut self) {
        self.menu.down();
    }

    fn add_device(&mut self, device: Device) {
        self.state.devices.insert(device.address, device);
    }

    fn remove_device(&mut self, address: Address) {
        self.state.devices.remove(&address);
    }
}

pub fn quick_menu() -> Box<dyn Menu> {
    Box::new(
        TableMenu::new(vec![], [Constraint::Fill(100)])
            .with_header(Row::new([Cell::new("Options")]))
            .with_ticker(|items, app_state| {
                items.clear();
                if app_state.player.is_paused() && !app_state.player.empty() {
                    items.push(AppEvent::Resume);
                }
                if !app_state.player.is_paused() && !app_state.player.empty() {
                    items.push(AppEvent::Pause);
                }
                Ok(())
            }),
    )
}
