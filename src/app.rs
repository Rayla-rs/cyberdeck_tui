use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use crate::app_actions::{AppAction, AppOnce};
use crate::blt_client::{BltClient, Device};
use crate::event::BltEvent;
use crate::machine::Instruction;
use crate::menus::menu::{Menu, NavigationResult};
use crate::trace_dbg;
use crate::{
    audio_player::AudioPlayer,
    config::Config,
    event::{AppEvent, Event, EventHandler},
    machine::Machine,
    menus::quick_menu::QuickMenu,
};
use bluer::{Address, Session};
use bluetui::app::AppResult;
use ratatui::widgets::ListItem;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use tracing::{Level, event, trace};

#[derive(PartialEq, Eq)]
pub enum Focus {
    MachineMenu,
    QuickMenu,
}

pub struct AppState {
    pub player: AudioPlayer,
    pub config: Config, //blt session
    pub blt_client: BltClient,
    pub devices: HashMap<Address, Device>,
}

/// Application.
pub struct App {
    pub context: String,
    pub state: AppState,
    pub machine: Machine,
    pub quick_menu: QuickMenu,
    pub focus: Focus,
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> AppResult<Self> {
        Ok(Self {
            context: format!("{}@{}", whoami::username(), whoami::devicename()),
            state: AppState {
                player: AudioPlayer::new(),
                config: Config::new()?,
                blt_client: BltClient::new().await?,
                devices: HashMap::default(),
            },
            machine: Machine::new(),
            quick_menu: QuickMenu::new(),
            focus: Focus::MachineMenu,
            running: true,
            events: EventHandler::new(),
        })
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> AppResult<()> {
        while self.running && self.machine.is_running() {
            // YES!!! I know this is janky as fuck!
            // Its to much of a pain tho to get ownership to work with actions and
            // The quick menu must consume them when returning with its enter
            // implimentation
            //
            // Unless it causes any major bugs or a better solution is found ur
            // better of rehidrating the Sahara with those tears!
            // (I spent way to long trying to find a nice way to do this)
            self.quick_menu.actions = self.machine.last_mut().get_quick_actions();
            if self.quick_menu.state.selected().is_none() {
                self.quick_menu.state.select_first();
            }

            self.state.player.tick();

            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(), // change to async
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
    pub fn tick(&mut self) {
        self.machine.tick(&mut self.state).ok().unwrap();

        // validate cursor location
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn enter(&mut self) -> AppResult<()> {
        let action = match self.focus {
            Focus::MachineMenu => self.machine.last_mut().enter()?,
            Focus::QuickMenu => self.quick_menu.enter()?,
        };
        self.handel_action(action)
    }

    pub fn up(&mut self) {
        match self.focus {
            Focus::MachineMenu => {
                let _ = self.machine.last_mut().up();
            }
            Focus::QuickMenu => match self.quick_menu.up() {
                NavigationResult::Previous => self.focus = Focus::MachineMenu,
                _ => {}
            },
        }
    }

    pub fn down(&mut self) {
        match self.focus {
            Focus::MachineMenu => match self.machine.last_mut().down() {
                NavigationResult::Next => {
                    self.focus = Focus::QuickMenu;
                }
                _ => {}
            },
            Focus::QuickMenu => {
                let _ = self.quick_menu.down();
            }
        }
    }

    fn handel_action(&mut self, action: AppAction) -> AppResult<()> {
        // Trace
        trace!(?action);

        match action {
            AppAction::MachineAction(instruction) => self.machine.handle_instruction(instruction),
            AppAction::StateAction(mutator) => {
                mutator.mutate_state(&mut self.state);
            }
            AppAction::Once(once) => once.once(),
        }
        Ok(())
    }

    fn add_device(&mut self, device: Device) {
        self.state.devices.insert(device.address, device);
    }

    fn remove_device(&mut self, address: Address) {
        self.state.devices.remove(&address);
    }
}
