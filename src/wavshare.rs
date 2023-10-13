use std::convert::Infallible;
use epd_waveshare::epd4in2::{Display4in2, Epd4in2, DEFAULT_BACKGROUND_COLOR};
use epd_waveshare::prelude::*;
use linux_embedded_hal::{Delay, Spidev, spidev, SysfsPin};
use linux_embedded_hal::spidev::SpidevOptions;
use linux_embedded_hal::sysfs_gpio::Direction;
use crate::Output;


fn spi() -> Spidev {
    let mut spi = Spidev::open("/dev/spidef0.0").expect("Failed to open SPI dir");
    let options = SpidevOptions::new().bits_per_word(8).max_speed_hz(4_000_000).mode(spidev::SpiModeFlags::SPI_MODE_0).build();
    spi.configure(&options).expect("spi configuration failed");
    spi
}

fn cs_pin() -> SysfsPin {
    let cs = SysfsPin::new(26); //BCM7 CE0
    cs.export().expect("cs export failed");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction failed");
    cs.set_value(1).expect("CS Value set to 1 failed");
    cs
}

fn busy_pin() -> SysfsPin {
    let busy = SysfsPin::new(5); //pin 29
    busy.export().expect("busy export failed");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction failed");
    busy
}


fn dc_pin() -> SysfsPin {
    let dc = SysfsPin::new(6); //pin 31 //bcm6
    dc.export().expect("dc export failed");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction failed");
    dc.set_value(1).expect("dc Value set to 1 failed");
    dc
}

fn rst_pin() -> SysfsPin {
    let rst = SysfsPin::new(16); //pin 36 //bcm16
    rst.export().expect("rst export failed");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction failed");
    rst.set_value(1).expect("rst Value set to 1 failed");
    rst
}

pub struct Wavshare {
    window: Epd4in2<Spidev, SysfsPin, SysfsPin, SysfsPin, SysfsPin, Delay>,
    delay: Delay,
    spi: Spidev,
}

impl Default for Wavshare {
    fn default() -> Self {
        let mut delay = Delay {};
        let mut spi = spi();
        let busy = busy_pin();
        let dc = dc_pin();
        let cs = cs_pin();
        let rst = rst_pin();
        let mut epd = Epd4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("failed to init e-ink display");

        Wavshare{window: epd, delay, spi}
    }
}

impl Output<Display4in2> for Wavshare {
    fn get_display(&self) -> Display4in2 {
        Display4in2::default()
    }

    fn clear_display(&self, draw_target: &mut Display4in2) -> Result<(), Infallible> {
        Ok(draw_target.clear_buffer(DEFAULT_BACKGROUND_COLOR))
    }

    fn draw(&mut self, draw_target: &mut Display4in2) {
        self.window.update_and_display_new_frame(&mut self.spi, draw_target.buffer(), &mut self.delay).expect("catastrophic frame update failure");
    }
}
