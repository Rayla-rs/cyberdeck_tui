use std::fmt::Display;

use crate::{
    AppResult,
    app_actions::{AppAction, PlayPlaylist},
    machine::Instruction,
    playlist::Playlist,
};

use super::menu::{Menu, MenuState};
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use linux_embedded_hal::{
    Delay, SpidevBus, SysfsPin,
    spidev::{SpiModeFlags, SpidevOptions},
    sysfs_gpio::Pin,
};

use mfrc522::Mfrc522;
use mfrc522::comm::blocking::spi::SpiInterface;

use ratatui::{
    prelude::*,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState},
};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter, VariantArray};

// Options -> read to buffer
// Write from buffer
// Write default 0u8

// TODO popup menu for res of action

#[derive(Display, EnumIter, VariantArray, EnumCount)]
enum Mfrc522Options {
    Read,
    Write,
    Clear,
}

impl<'a> Into<ListItem<'a>> for Mfrc522Options {
    fn into(self) -> ListItem<'a> {
        ListItem::from(format!("{}", self))
    }
}

#[derive(Debug)]
pub struct Mfrc522Menu {
    state: ListState,
    buffer: Option<u8>,
}

impl Display for Mfrc522Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("mfrc522")
    }
}

impl Menu for Mfrc522Menu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        Mfrc522Options::COUNT
    }

    fn get_quick_actions(&self) -> Vec<AppAction> {
        vec![AppAction::MachineAction(Instruction::Pop)]
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        ratatui::widgets::Clear::default().render(area, buf);

        let list = List::new(Mfrc522Options::iter())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            })
            .yellow()
            .block(Block::bordered());
        StatefulWidget::render(list, area, buf, &mut self.state);

        Ok(area)
    }
}

/*
RFID    Pi Pin  Pin Name
SDA     37      GPIO26 (dtparam sets this as CE0)
SCK     40      SPI1 SCLK
MOSI    38      SPI1 MOSI
MISO    35      SPI1 MISO
IRQ     29      GPIO5
GND     39      Ground
RST     31      GPIO6
3.3V    17      VDD_3V3
*/

impl Mfrc522Menu {
    pub fn new() -> AppResult<Self> {
        let mut state = ListState::default();
        state.select_first();

        let mut delay = Delay;
        let mut spi = SpidevBus::open("/dev/spidev0.1")?;
        let options = SpidevOptions::new()
            .max_speed_hz(1_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;

        // software-controlled chip select pin
        let pin = SysfsPin::new(26);
        pin.export()?;
        while !pin.is_exported() {}
        delay.delay_ms(500u32); // delay sometimes necessary because `is_exported()` returns too early?
        let pin = pin.into_output_pin(embedded_hal::digital::PinState::High)?;

        let spi = ExclusiveDevice::new(spi, pin, Delay)?;
        let itf = SpiInterface::new(spi);
        let mut mfrc522 = Mfrc522::new(itf).init()?;

        let vers = mfrc522.version()?;

        Ok(Self {
            state,
            buffer: None,
        })
    }
}
