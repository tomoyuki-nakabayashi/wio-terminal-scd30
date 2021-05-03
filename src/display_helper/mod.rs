use arrayvec::ArrayString;
use wio_terminal::LCD;
use core::fmt::Write;
use embedded_graphics::{egtext, fonts::{Font24x32, Font12x16, Text}, geometry::Point, pixelcolor::Rgb565, prelude::*, style::TextStyle, text_style};

pub fn draw_text(mut display: LCD) -> LCD {
    Text::new("Air Quality", Point::new(20, 30))
        .into_styled(TextStyle::new(Font24x32, Rgb565::BLUE))
        .draw(&mut display)
        .unwrap();

    Text::new("Carbon Dioxide:", Point::new(5, 90))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    Text::new("Temperature:", Point::new(5, 130))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    Text::new("Humidity:", Point::new(5, 170))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    display
}

pub fn draw_numbers(
    value: f32,
    unit: &str,
    position: (i32, i32),
    mut display: LCD,
) -> LCD {
    let mut buf = ArrayString::<[_; 12]>::new();
    write!(&mut buf, "{:.2} {}", value, unit).expect("Failed to write to buffer");
    egtext!(
        text = &buf,
        top_left = position,
        style = text_style!(font = Font12x16, text_color = Rgb565::GREEN,)
    )
    .draw(&mut display)
    .unwrap();

    display
}
