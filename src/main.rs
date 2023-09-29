use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    #[cfg(feature = "simulator")]
    simulator::run();

    Ok(())
}


#[cfg(feature = "simulator")]
mod simulator {
    use std::error::Error;
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};
    use embedded_graphics::draw_target::DrawTarget;
    use embedded_graphics::Drawable;
    use embedded_graphics::geometry::{Point, Size};
    use embedded_graphics::mono_font::ascii::FONT_6X9;
    use embedded_graphics::mono_font::MonoTextStyle;
    use embedded_graphics::pixelcolor::BinaryColor;
    use embedded_graphics::primitives::{Primitive, PrimitiveStyle, Rectangle};
    use embedded_graphics::text::Text;
    use embedded_graphics_simulator::{BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window};

    pub fn run() -> Result<(), Box<dyn Error>> {
        // setup display
        let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

        // setup styles
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        // setup window
        let output_settings = OutputSettingsBuilder::new().scale(1)
            .theme(BinaryColorTheme::Default)
            .build();
        let mut window = Window::new("Hello World", &output_settings);


        // setup timers
        let update_interval = Duration::from_secs(10); // how often we refresh data
        let interval = Duration::from_millis(500); // how often we paint

        // when our next paint is -- this keeps our interval consistent at 500ms no matter how long the execution takes (as long as its less than 500ms)
        let mut next_time = Instant::now() + interval;
        let mut next_data_update = Instant::now() + update_interval; // when our next data update is

        let mut handle: Option<JoinHandle<()>> = None; // data update thread handles

        // do work
        loop {
            println!("tick");
            // clear display
            display.clear(BinaryColor::Off)?;

            // draw UI elements
            Rectangle::new(Point::new(5, 5), Size::new(390, 290))
                .into_styled(line_style)
                .draw(&mut display)?;

            // if we are doing a data update lets trigger it in a separate thread
            if next_data_update <= Instant::now() {
                match &handle {
                    Some(v) => {
                        println!("Checking thread");
                        if v.is_finished() {
                            println!("Thread over");
                            next_data_update += update_interval;
                            handle = None;
                        } else {
                            println!("updating in progress");
                            Text::new("Updating data...", Point::new(5, 5), text_style).draw(&mut display)?;
                        }
                    }
                    None => {
                        println!("No thread spawning");
                        Text::new("Updating data...", Point::new(5, 5), text_style).draw(&mut display)?;
                        handle = Some(std::thread::spawn(|| {
                            println!("Thread spawned");
                            std::thread::sleep(Duration::from_secs(5));
                            println!("Thread finished!");
                        }));
                    }
                }
            } else {
                println!("updating done");
                Text::new("Data refresh in...", Point::new(5, 5), text_style).draw(&mut display)?;
            }

            // paint UI
            window.update(&display);

            // sleep until next loop
            std::thread::sleep(next_time - Instant::now());
            next_time += interval; // set next loop runtime
        }
    }
}

