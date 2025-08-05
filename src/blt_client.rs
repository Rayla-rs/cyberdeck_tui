use std::sync::Arc;

use bluer::{Adapter, Session};
use bluer::{Address, Device as BTDevice};
use tracing::trace;

use crate::AppResult;
use crate::app_actions::AppAction;

pub struct BltClient {
    pub session: Session,
    pub adapter: Adapter,
    pub devices: Vec<Device>,
}

#[derive(Debug)]
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

    pub async fn test(&self) {
        match self.get_all_devices().await {
            Ok(res) => {
                for res in res {
                    let res = format!("{:?}", res);
                    trace!(res)
                }
            }
            Err(_) => {
                trace!("err");
            }
        }
    }
}
