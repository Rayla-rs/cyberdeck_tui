use std::sync::Arc;

use bluer::{Adapter, Session};
use bluer::{Address, Device as BTDevice};
use ratatui::layout::Constraint;
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};

use crate::app::quick_menu;
use crate::event::AppEvent;
use crate::menus::{Item, LinkedMenu, MenuFrame, TableMenu};

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
    pub is_connected: bool,
    pub is_trusted: bool,
    pub battery_percentage: Option<u8>,
}

impl Device {
    pub async fn pair(&self) -> bluer::Result<()> {
        self.bt_device.pair().await
    }

    fn data(&self) -> [String; 5] {
        [
            self.alias.clone(),
            self.address.to_string(),
            format!("{}", self.is_paired),
            format!("{}", self.is_connected),
            format!("{}", self.is_trusted),
        ]
    }

    pub async fn new(adapter: &Adapter, address: Address) -> bluer::Result<Self> {
        let bt_device = adapter.device(address)?;
        let alias = bt_device.alias().await?;
        let is_paired = bt_device.is_paired().await?;
        let is_connected = bt_device.is_connected().await?;
        let is_trusted = bt_device.is_trusted().await?;
        let battery_percentage = bt_device.battery_percentage().await?;

        Ok(Device {
            bt_device,
            address,
            alias,
            is_paired,
            is_trusted,
            is_connected,
            battery_percentage,
        })
    }
}

impl Item for Device {}

impl Into<AppEvent> for Device {
    fn into(self) -> AppEvent {
        AppEvent::Push(Arc::new(move || {
            let mut options = Vec::new();
            options.push(if self.is_connected {
                AppEvent::Disconnect(self.clone())
            } else {
                AppEvent::Connect(self.clone())
            });
            options.push(if self.is_trusted {
                AppEvent::Untrust(self.clone())
            } else {
                AppEvent::Trust(self.clone())
            });

            LinkedMenu::new(Box::new(MenuFrame::new([
                Box::new(TableMenu::new(options, [Constraint::Fill(100)])),
                quick_menu(),
            ])))
        }))
    }
}

impl Into<Row<'static>> for Device {
    fn into(self) -> Row<'static> {
        self.data()
            .iter()
            .map(|elem| Cell::from(Text::from(format!("{elem}"))))
            .collect()
    }
}

#[derive(Clone)]
pub struct BluetoothItem;

impl Item for BluetoothItem {}

impl BluetoothItem {
    pub fn to_menu(self) -> TableMenu<BluetoothItem, [Constraint; 1]> {
        TableMenu::new(vec![self], [Constraint::Fill(100)])
    }
}

impl Into<AppEvent> for BluetoothItem {
    fn into(self) -> AppEvent {
        AppEvent::Push(Arc::new(|| create_blt_menu()))
    }
}

impl<'a> Into<Row<'a>> for BluetoothItem {
    fn into(self) -> Row<'a> {
        Row::new([Cell::new("Bluetooth")])
    }
}

pub fn create_blt_menu() -> LinkedMenu {
    LinkedMenu::new(Box::new(MenuFrame::new([
        Box::new(
            TableMenu::new(
                vec![],
                [
                    Constraint::Min(5),
                    Constraint::Length(17),
                    Constraint::Length(6),
                    Constraint::Length(9),
                    Constraint::Length(7),
                ],
            )
            .with_header(Row::new([
                Cell::new("Alias"),
                Cell::new("Adress"),
                Cell::new("Paired"),
                Cell::new("Connected"),
                Cell::new("Trusted"),
            ]))
            .with_ticker(|items, app_state| {
                items.clear();
                let mut devices = app_state.cloned_devices();
                items.append(&mut devices);
                Ok(())
            }),
        ),
        quick_menu(),
    ])))
}
