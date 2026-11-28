use core::{ops::DerefMut, panic::PanicInfo};

use alloc::format;
use embedded_graphics::{
    Drawable, draw_target::DrawTarget, geometry::{Point, Size},
    mono_font::{MonoTextStyle, ascii}, pixelcolor::{Rgb888, RgbColor},
    primitives::Rectangle, text::Text
};
use crate::gfx;

/// Renders a panic message if ModOS is already in graphics mode.
pub fn panic(info: &PanicInfo) {
    if let Some(handle) = gfx::HANDLE.as_mut().as_mut() {
        let message = format!(
"
--------------------------------------------------
MODOS PANICKED!
RUNTIME HAS BEEN SUSPENDED.
LOOK BELOW FOR MORE DETAILS, AND THEN HOLD POWER
UNTIL THE SYSTEM IS COMPLETELY OFF.
--------------------------------------------------

{info}
"
        );
        let style = MonoTextStyle::new(&ascii::FONT_10X20, Rgb888::WHITE);
        let text = Text::new(&message, Point::new(30, 30), style);
        let res = handle.mode().resolution();
        let _ = handle.fill_solid(&Rectangle::new(
            Point::zero(), Size::new(res.0 as u32, res.1 as u32)), Rgb888::BLACK
        );
        let _ = text.draw(handle.deref_mut());
        handle.flush();
    }
}