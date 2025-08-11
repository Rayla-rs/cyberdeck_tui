// TODO rename to device
use std::fmt::Display;

use bluer::{Adapter, Session};
use bluer::{Address, Device as BTDevice};
use ratatui::buffer::Buffer;
use ratatui::layout::{Margin, Rect};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{
    Block, Cell, HighlightSpacing, List, ListItem, ListState, Row, StatefulWidget, Widget,
};
use strum::{EnumCount, IntoEnumIterator, VariantArray};
use strum_macros::{Display, EnumCount, EnumIter, VariantArray};
use tracing::trace;

use crate::AppResult;
use crate::app_actions::{AppAction, AppOnce, PairDevice};
use crate::machine::Instruction;
use crate::menus::menu::{Menu, MenuState};

pub struct BltClient {
    pub session: Session,
    pub adapter: Adapter,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub bt_device: BTDevice,
    pub address: Address,
    pub alias: String,
    pub is_paired: bool,
    pub is_trusted: bool,
    pub battery_percentage: Option<u8>,
}

impl Device {
    pub async fn pair(&self) -> bluer::Result<()> {
        self.bt_device.pair().await
    }

    fn data(&self) -> [String; 4] {
        [
            self.alias.clone(),
            self.address.to_string(),
            format!("{}", self.is_paired),
            format!("{}", self.is_trusted),
        ]
    }

    pub async fn new(adapter: &Adapter, address: Address) -> bluer::Result<Self> {
        let bt_device = adapter.device(address)?;
        let alias = bt_device.alias().await?;
        let is_paired = bt_device.is_paired().await?;
        let is_trusted = bt_device.is_trusted().await?;
        let battery_percentage = bt_device.battery_percentage().await?;

        Ok(Device {
            bt_device,
            address,
            alias,
            is_paired,
            is_trusted,
            battery_percentage,
        })
    }
}

impl<'a> Into<Row<'a>> for &'a Device {
    fn into(self) -> Row<'a> {
        self.data()
            .iter()
            .map(|elem| Cell::from(Text::from(format!("{elem}"))))
            .collect()
    }
}

impl BltClient {
    pub async fn new() -> AppResult<Self> {
        let session = Session::new().await?;
        let adapter = session.default_adapter().await?;

        Ok(Self {
            session,
            adapter,
            devices: vec![],
        })
    }

    async fn get_all_devices(&self) -> AppResult<Vec<Device>> {
        let mut devices: Vec<Device> = Vec::new();

        for address in self.adapter.device_addresses().await? {
            let bt_device = self.adapter.device(address)?;
            let alias = bt_device.alias().await?;
            let is_paired = bt_device.is_paired().await?;
            let is_trusted = bt_device.is_trusted().await?;
            let battery_percentage = bt_device.battery_percentage().await?;

            devices.push(Device {
                bt_device,
                address,
                alias,
                is_paired,
                is_trusted,
                battery_percentage,
            });
        }

        Ok(devices)
    }

    pub async fn test(&mut self) {
        match self.get_all_devices().await {
            Ok(res) => {
                self.devices = res;
            }
            Err(_) => {
                trace!("err");
            }
        }
    }
}

// Device options menu

#[derive(Display, EnumIter, VariantArray, EnumCount)]
enum DeviceOptions {
    Pair,
    Trust,
}

impl<'a> Into<ListItem<'a>> for DeviceOptions {
    fn into(self) -> ListItem<'a> {
        ListItem::from(format!("{}", self))
    }
}

#[derive(Debug)]
pub struct DeviceMenu {
    state: ListState,
    device: Device,
}

impl Display for DeviceMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("bltdevice({})", self.device.alias))
    }
}

impl Menu for DeviceMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        DeviceOptions::COUNT
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into()]
    }

    fn enter(&mut self) -> AppResult<crate::app_actions::AppAction> {
        // TODO -> trust, pair, connect, untrust
        // Extra: async popup menu
        Ok(
            match DeviceOptions::VARIANTS[self.state.selected().unwrap()] {
                DeviceOptions::Pair => {
                    AppAction::Once(Box::new(PairDevice::new(self.device.clone())))
                }
                DeviceOptions::Trust => AppAction::Once(Box::new(Trust::new(self.device.clone()))),
            },
        )
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        let area = area.inner(Margin {
            horizontal: 5,
            vertical: 5,
        });

        ratatui::widgets::Clear::default().render(area, buf);

        // todo -> make smaller hehe

        // Layout::new(direction, constraints)

        // paragraph of data

        // List of actions

        let list = List::new(DeviceOptions::iter())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            })
            .yellow()
            .block(Block::bordered());
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);

        Ok(area)
    }
}

impl DeviceMenu {
    pub fn new(device: Device) -> Self {
        let mut state = ListState::default();
        state.select_first();
        Self { state, device }
    }
}

#[derive(Debug)]
struct Trust {
    device: Device,
}

impl Display for Trust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("trust({:?})", self.device))
    }
}

impl AppOnce for Trust {
    fn once(self: Box<Self>) {
        tokio::spawn(async move {
            let _ = self.device.bt_device.set_trusted(true);
        });
    }
}

impl Trust {
    fn new(device: Device) -> Self {
        Self { device }
    }
}
