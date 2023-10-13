use std::convert::Infallible;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Size};
use embedded_graphics_simulator::{BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window};
use crate::Output;

pub struct Simulator {
    window: Window,
}

impl Default for Simulator {
    fn default() -> Self {
        let output_settings = OutputSettingsBuilder::new().scale(1)
            .theme(BinaryColorTheme::LcdWhite)
            .build();
        let window = Window::new("e-ink-pi-dash", &output_settings);

        Self { window }
    }
}

impl Output<SimulatorDisplay<BinaryColor>> for Simulator {
    fn get_display(&self) -> SimulatorDisplay<BinaryColor> {
        SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300)).to_owned()
    }

    fn clear_display(&self, draw_target: &mut SimulatorDisplay<BinaryColor>) -> Result<(), Infallible> {
        draw_target.clear(BinaryColor::Off)
    }

    fn draw(&mut self, draw_target: &mut SimulatorDisplay<BinaryColor>) {
        self.window.update(draw_target)
    }
}