#![no_main]
#![no_std]

use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{
    Circle, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, Triangle,
};
use embedded_graphics::text::Alignment;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::*;
use embedded_graphics_core::primitives::Rectangle;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi_graphics_driver::{UefiDisplay, UefiDisplayError};

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bt = system_table.boot_services();
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = bt
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();

    let mode = gop.current_mode_info();

    // 创建屏幕
    let mut display = UefiDisplay::new(mode.resolution());

    // 写入数据
    test_uefi_driver(&mut display).expect("unsupported error");
    // 刷新屏幕
    display.flush(&mut gop).expect("flush error");

    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn test_uefi_driver(display: &mut UefiDisplay) -> Result<(), UefiDisplayError> {
    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::new(255, 0, 0), 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Rgb888::new(255, 0, 0), 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::new(255, 0, 0))
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(Rgb888::new(255, 0, 0));
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 0, 0));

    let y_offset = 10;

    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(display)?;

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + y_offset),
        Point::new(16 + 16, 16 + y_offset),
        Point::new(16 + 8, y_offset),
    )
    .into_styled(thin_stroke)
    .draw(display)?;

    // Draw a filled square
    Rectangle::new(Point::new(52, y_offset), Size::new(16, 16))
        .into_styled(fill)
        .draw(display)?;

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, y_offset), 17)
        .into_styled(thick_stroke)
        .draw(display)?;

    // Draw centered text.
    let text = "embedded-graphics";
    embedded_graphics::text::Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(display)?;

    Ok(())
}
