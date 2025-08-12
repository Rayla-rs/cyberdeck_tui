use bluer::{Adapter, Session};
use bluer::{Address, Device as BTDevice};
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};

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
