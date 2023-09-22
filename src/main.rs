mod gmail_client;
mod shopify_client;

use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, Rectangle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let shopify_refresh_rate: u16 = env::var("SHOPIFY_REFRESH_RATE")?.parse().expect("SHOPIFY_REFRESH_RATE must be of type u16");
    let gmail_refresh_rate: u16 = env::var("GMAIL_REFRESH_RATE")?.parse().expect("SHOPIFY_REFRESH_RATE must be of type u16");
    let shopify_api_key = env::var("SHOPIFY_API_KEY").expect("SHOPIFY_API_KEY is required.");
    let gmail_api_key = env::var("GMAIL_API_KEY").expect("GMAIL_API_KEY is required.");




    Ok(())
}

// #[cfg(feature="simulator")]
// fn test() -> Result<(), core::convert::Infallible> {
//     let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));
//
//     let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
//     let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
//
//     Line::new(Point::new(48, 16), Point::new(8, 16))
//         .into_styled(line_style)
//         .draw(&mut display)?;
//
//     Line::new(Point::new(48, 16), Point::new(64, 32))
//         .into_styled(line_style)
//         .draw(&mut display)?;
//
//     Rectangle::new(Point::new(5, 5), Size::new(390, 290))
//         .into_styled(line_style)
//         .draw(&mut display)?;
//
//     Text::new("Hello World!", Point::new(5, 5), text_style).draw(&mut display)?;
//
//     let output_settings = OutputSettingsBuilder::new().scale(2)
//         .theme(BinaryColorTheme::Default)
//         .build();
//     let mut window = Window::new("Hello World", &output_settings);
//
//     'running: loop {
//         window.update(&display)
//     }
//     println!("first paint");
//
//     window.show_static(&display);
//
//
//     sleep(Duration::from_secs(5));
//
//     Text::new("Hello World2", Point::new(35, 35), text_style).draw(&mut display)?;
//     window.update(&display);
//     println!("Updated");
//
//     Ok(())
// }

