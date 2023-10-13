use std::time::Instant;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::mono_font::ascii::{FONT_4X6, FONT_9X15_BOLD, FONT_6X10};
use embedded_graphics::mono_font::{MonoTextStyle};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Primitive};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_text::alignment::{HorizontalAlignment, VerticalAlignment};
use embedded_text::style::{HeightMode, TextBoxStyle, TextBoxStyleBuilder, VerticalOverdraw};
use embedded_text::TextBox;


pub fn sm_text_style() -> MonoTextStyle<'static, BinaryColor> {
    MonoTextStyle::new(&FONT_4X6, BinaryColor::On)
}

pub fn md_text_style() -> MonoTextStyle<'static, BinaryColor> {
    MonoTextStyle::new(&FONT_6X10, BinaryColor::On)
}

pub fn lg_text_style_bold() -> MonoTextStyle<'static, BinaryColor> {
    MonoTextStyle::new(&FONT_9X15_BOLD, BinaryColor::On)
}

pub fn stroke_style() -> PrimitiveStyle<BinaryColor>  {
    PrimitiveStyle::with_stroke(BinaryColor::On, 1)
}

static BOX_SIZE: Size = Size::new(95, 73);
static INNER_BOX_SIZE: Size = Size::new(90, 68);

static LABEL_STYLE: TextBoxStyle = TextBoxStyleBuilder::new().height_mode(HeightMode::Exact(VerticalOverdraw::Hidden)).alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Bottom).build();
static VALUE_STYLE: TextBoxStyle = TextBoxStyleBuilder::new().height_mode(HeightMode::Exact(VerticalOverdraw::Hidden)).alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Middle).build();

static EMAIL_STYLE: TextBoxStyle = TextBoxStyleBuilder::new().height_mode(HeightMode::Exact(VerticalOverdraw::FullRowsOnly)).alignment(HorizontalAlignment::Left).build();

pub fn draw_border<D>(display: &mut D) -> Result<(), D::Error> where D: DrawTarget<Color=BinaryColor> {
    Rectangle::new(Point::new(2, 2), Size::new(396, 296))
        .into_styled(stroke_style())
        .draw(display)?;
    Ok(())
}

pub fn draw_update_text<D>(display: &mut D, next_update_time: Option<Instant>) -> Result<(), D::Error> where D: DrawTarget<Color=BinaryColor> {
    let text = match next_update_time {
        None => "Updating data...".to_string(),
        // we add +1 to the time because we don't want to show 0, human readable vs machine readable countdown
        Some(time) => format!("Data refresh in {} seconds...", time.duration_since(Instant::now()).as_secs() + 1)
    };
    Text::new(text.as_str(), Point::new(5, 10), sm_text_style()).draw(display)?;

    Ok(())
}

pub fn draw_gmail_box<D>(display: &mut D) -> Result<(), D::Error> where D: DrawTarget<Color=BinaryColor> {
    Rectangle::new(Point::new(5, 15), Size::new(195, 280)).into_styled(stroke_style()).draw(display)?;
    Text::new("Unread Email", Point::new(10, 28), lg_text_style_bold()).draw(display)?;
    let mut next = 35;
    for _ in 0..13 {
        next = draw_email(display, next, "some_email@gmail.com", "Some crazy email subject that overflows the bounds of the text box")?;
    }
    Ok(())
}

pub fn draw_shopify_boxes<D>(display: &mut D) -> Result<(), D::Error> where D: DrawTarget<Color=BinaryColor> {
    // Row 1 Box 1
    draw_box(display, Point::new(202, 15), "Today's Sales", "$1,250.88")?;

    // Row 1 Box 2
    draw_box(display, Point::new(300, 15), "Sales Month", "$10,250.04")?;


    // Row 2 Box 1
    draw_box(display, Point::new(202, 93), "Visitors Today", "102")?;

    // Row 2 Box 2
    draw_box(display, Point::new(300, 93), "Visitors Month", "150,021")?;

    // Row 3 Box 1
    draw_box(display, Point::new(202, 168), "Orders Today", "18")?;

    // Row 3 Box 2
    draw_box(display, Point::new(300, 168), "Orders Month", "105")?;

    // Currently Visiting Text
    let bounds = Rectangle::new(Point::new(205, 248), Size::new(190, 0));
    let textbox_style = TextBoxStyleBuilder::new().height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle).build();

    TextBox::with_textbox_style("5\npeople currently browsing", bounds, lg_text_style_bold(), textbox_style).draw(display)?;

    Ok(())
}

fn draw_box<D>(display: &mut D, top_left: Point, label_text: &str, value_text: &str) -> Result<(), D::Error> where D: DrawTarget<Color=BinaryColor> {
    Rectangle::new(top_left, BOX_SIZE).into_styled(stroke_style()).draw(display)?;
    let bounds = Rectangle::new(top_left, INNER_BOX_SIZE);
    TextBox::with_textbox_style(label_text, bounds, md_text_style(), LABEL_STYLE).draw(display)?;
    TextBox::with_textbox_style(value_text, bounds, lg_text_style_bold(), VALUE_STYLE).draw(display)?;
    Ok(())
}

fn draw_email<D>(display: &mut D, start_y: i32, sender: &str, subject: &str) -> Result<i32, D::Error> where D: DrawTarget<Color=BinaryColor> {
    let address_bounds = Rectangle::new(Point::new(10, start_y), Size::new(190, 15));
    TextBox::with_textbox_style(sender, address_bounds, sm_text_style(), EMAIL_STYLE).draw(display)?;
    let subject_bounds = Rectangle::new(Point::new(10, start_y + 5), Size::new(190, 15));
    TextBox::with_textbox_style(subject, subject_bounds, md_text_style(), EMAIL_STYLE).draw(display)?;

    Ok(start_y + 20)
}