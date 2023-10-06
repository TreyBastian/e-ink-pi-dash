use std::error::Error;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Size};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_simulator::{BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window};
use crate::ui::{draw_border, draw_gmail_box, draw_shopify_boxes, draw_update_text};
use crate::app::App;


impl App {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // setup display
        let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

        // setup window
        let output_settings = OutputSettingsBuilder::new().scale(1)
            .theme(BinaryColorTheme::Default)
            .build();
        let mut window = Window::new("e-ink-pi-dash", &output_settings);

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
            window.update(&display);

            // sleep until next loop
            std::thread::sleep(next_time - Instant::now());
            next_time += interval; // set next loop runtime
        }
    }
}

