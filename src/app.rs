use std::error::Error;
use std::time::{Duration, Instant};
use crate::Output;
use crate::shopify::{Shopify};
use crate::ui::{draw_border, draw_gmail_box, draw_shopify_boxes, draw_update_text};

pub struct App {
    shopify_client: Shopify,
    update_interval: Duration,
    #[cfg(feature = "simulator")]
    output: crate::simulator::Simulator,
    #[cfg(feature = "wavshare")]
    output: crate::wavshare::Wavshare,
}


impl App {
    pub fn new(shopify_client: Shopify, update_interval: Duration) -> Self {
        #[cfg(feature = "simulator")]
            let output = crate::simulator::Simulator::default();

        #[cfg(feature = "wavshare")]
            let output = crate::wavshare::Wavshare::default();

        Self { shopify_client, update_interval, output }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut display = self.output.get_display();

        let interval = Duration::from_millis(500); // how often we paint

        // when our next paint is -- this keeps our interval consistent at 500ms no matter how long the execution takes (as long as its less than 500ms)
        let mut next_time = Instant::now() + interval;
        let mut next_data_update = Instant::now() + self.update_interval; // when our next data update is

        // do work
        loop {
            // clear display
            self.output.clear_display(&mut display)?;

            // draw ui elements
            draw_border(&mut display)?;
            draw_gmail_box(&mut display)?;
            draw_shopify_boxes(&mut display)?;

            // do we need to update our data?
            if next_data_update <= Instant::now() {
                // draw updating text
                draw_update_text(&mut display, None)?;
                self.output.draw(&mut display);

                // do the work

                //done!
                next_data_update = Instant::now() + self.update_interval;
                continue;
            } else {
                draw_update_text(&mut display, Some(next_data_update))?;
            }

            // paint UI
            self.output.draw(&mut display);

            // sleep until next tick
            std::thread::sleep(next_time - Instant::now());
            next_time += interval;
        }
    }
}