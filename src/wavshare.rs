use std::error::Error;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Size};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_simulator::{BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window};
use epd_waveshare::epd4in2::{Display4in2, Epd4in2};
use epd_waveshare::prelude::{QuickRefresh, WaveshareDisplay};
use linux_embedded_hal::{Delay, Spidev, spidev, SysfsPin};
use linux_embedded_hal::spidev::SpidevOptions;
use serde::__private::ser::constrain;
use crate::ui::{draw_border, draw_gmail_box, draw_shopify_boxes, draw_update_text};
use crate::app::App;

pub fn spi() -> Spidev {
    let mut spi = Spidev::open("/dev/spidef0.0").expect("Failed to open SPI dir");
    let options = SpidevOptions::new().bits_per_word(8).max_speed_hz(4_000_000).mode(spidev::SpiModeFlags::SPI_MODE_0).build();
    spi.configure(&options).expect("spi configuration failed")
}

pub fn cs_pin() -> SysfsPin {
    let cs = SysfsPin::new(26); //BCM7 CE0
    cs.export().expect("cs export failed");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction failed");
    cs.set_value(1).expect("CS Value set to 1 failed")
}

pub fn busy_pin() -> SysfsPin {
    let busy = SysfsPin::new(5); //pin 29
    busy.export().expect("busy export failed");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction failed")
}


pub fn dc_pin() -> SysfsPin {
    let dc = SysfsPin::new(6); //pin 31 //bcm6
    dc.export().expect("dc export failed");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction failed");
    dc.set_value(1).expect("dc Value set to 1 failed")
}

pub fn rst_pin() -> SysfsPin {
    let rst = SysfsPin::new(16); //pin 36 //bcm16
    rst.export().expect("rst export failed");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction failed");
    rst.set_value(1).expect("rst Value set to 1 failed")
}

impl App {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // configure SPI
        let spi = spi();
        let busy = busy_pin();
        let dc = dc_pin();
        let cs = cs_pin();
        let rst = rst_pin();

        let mut delay = Delay {};

        // setup epd
        let mut epd = Epd4in2::new(&mut spi, busy, dc, rst, delay, None).expect("failed to init e-ink display");

        // setup display
        let mut display = Display4in2::default();

        // setup timers
        let interval = Duration::from_millis(500); // how often we paint

        // when our next paint is -- this keeps our interval consistent at 500ms no matter how long the execution takes (as long as its less than 500ms)
        let mut next_time = Instant::now() + interval;
        let mut next_data_update = Instant::now() + self.update_interval; // when our next data update is

        let mut handle: Option<JoinHandle<()>> = None; // data update thread handles

        // do work
        loop {
            // clear display
            display.clear(BinaryColor::Off)?;

            // draw UI elements
            draw_border(&mut display)?;

            draw_gmail_box(&mut display)?;
            draw_shopify_boxes(&mut display)?;

            // if we are doing a data update lets trigger it in a separate thread
            if next_data_update <= Instant::now() {
                match &handle {
                    Some(v) => {
                        if v.is_finished() {
                            next_data_update = Instant::now() + self.update_interval;
                            handle = None;
                        }
                    }
                    None => {
                        draw_update_text(&mut display, None)?;
                        handle = Some(std::thread::spawn(|| {
                            std::thread::sleep(Duration::from_secs(5));
                        }));
                    }
                }
                draw_update_text(&mut display, None)?;
            } else {
                draw_update_text(&mut display, Some(next_data_update))?;
            }

            // paint UI
            epd.update_and_display_new_frame(&mut spi, &display.buffer(), &mut delay)?;

            // sleep until next loop
            std::thread::sleep(next_time - Instant::now());
            next_time += interval; // set next loop runtime
        }
    }
}

