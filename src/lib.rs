use std::convert::Infallible;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;

pub mod ui;
pub mod shopify;

#[cfg(feature = "simulator")]
pub mod simulator;
pub mod app;

#[cfg(feature = "wavshare")]
pub mod wavshare;


pub trait Output<D> where D: DrawTarget<Color=BinaryColor> {
    fn get_display(&self) -> D;

    fn clear_display(&self, draw_target: &mut D) -> Result<(), Infallible>;
    fn draw(&mut self, draw_target: &mut D);
}