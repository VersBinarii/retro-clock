use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
    text::{Baseline, Text},
};
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

type Disp<I, S> = Ssd1306<I, S, BufferedGraphicsMode<S>>;

pub struct Display<I, S>
where
    S: DisplaySize,
{
    display: Disp<I, S>,
}

impl<I, S> Display<I, S>
where
    S: DisplaySize,
    I: WriteOnlyDataCommand,
{
    pub fn new(display: Disp<I, S>) -> Self {
        Self { display }
    }

    pub fn print_text(&mut self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();
        Text::with_baseline(text, Point::new(x, y), style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();

        self.display.flush().unwrap();
    }

    pub fn print_pointer(&mut self, v1: &Point, v2: &Point, v3: &Point) {
        Triangle::new(*v1, *v2, *v3)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.display.clear();
    }
}
