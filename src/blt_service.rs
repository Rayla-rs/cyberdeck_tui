use std::sync::Arc;

use bluer::{Adapter, AdapterEvent, Address, DiscoveryFilter, Session};
use futures::StreamExt;
use futures::lock::Mutex;
use futures::stream::BoxStream;
use tracing::{Level, debug};

use crate::trace_dbg;

pub struct BltService {
    pub adapter: Adapter,
    pub stream: Option<BoxStream<'static, AdapterEvent>>,
}

impl BltService {
    pub async fn init() -> color_eyre::Result<Arc<Mutex<Self>>> {
        let session = Session::new().await?;
        let adapter = session.default_adapter().await?;

        // Discovery filter
        let filter = DiscoveryFilter {
            transport: bluer::DiscoveryTransport::Auto,
            ..DiscoveryFilter::default()
        };
        adapter.set_discovery_filter(filter).await?;

        trace_dbg!("Bluetooth Init");

        Ok(Arc::new(Mutex::new(Self {
            adapter,
            stream: None,
        })))
    }

    async fn set_state(&mut self, enable: bool) -> color_eyre::Result<()> {
        let active = self.stream.is_some();

        match (enable, active) {
            (true, _) => {
                self.stream = Some(self.adapter.discover_devices().await?.boxed());
                trace_dbg!("BLT DISABLED");
            }
            (false, _) => {
                self.stream = None;
                trace_dbg!("BLT ENABLED");
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn enable(&mut self) -> color_eyre::Result<()> {
        self.set_state(true).await
    }

    pub async fn disable(&mut self) -> color_eyre::Result<()> {
        self.set_state(false).await
    }
}
