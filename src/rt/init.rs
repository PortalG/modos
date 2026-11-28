use core::ops::DerefMut;

use alloc::format;
use embedded_graphics::{Drawable, draw_target::DrawTarget, geometry::{Point, Size}, mono_font::{MonoTextStyle, ascii}, pixelcolor::{Rgb888, RgbColor}, primitives::Rectangle, text::Text};
use log::info;
use uefi::boot;
use crate::{fs, gfx, rt};

/// Initializes the runtime.
pub fn init() -> ! {
    info!("calculating cps...");
    let cps = rt::calculate_cps();
    info!("CPU sits at {} cycles per second.", cps);


    info!("setting up handles...");
    fs::init();
    gfx::init();

    let target = 60;
    let mut entropy = 0.0;

    let mut last = rt::cycle_count();
    
    let handle = gfx::HANDLE.get_mut();
    let resolution = gfx::HANDLE.get().mode().resolution();
    let style = MonoTextStyle::new(&ascii::FONT_6X10, Rgb888::BLACK);
    
    let mut time = 1.0;

    loop {
        let delta = rt::calculate_delta(&mut last, cps);
        rt::stall(target, &mut entropy, delta);

        time += delta;

        if (time as usize).is_multiple_of(10) {
            boot::stall(rt::SECOND);
        }

        let color = Rgb888::new(
            ((libm::sin(time * 0.5) + 1.0) * 255.0) as u8,
            ((libm::sin(time * 0.1) + 1.0) * 255.0) as u8,
            ((libm::sin(time * 0.3) + 1.0) * 255.0) as u8
        );
        handle.fill_solid(&Rectangle::new(Point::new(0, 0), Size::new(resolution.0 as u32, resolution.1 as u32)), color).unwrap();
        let rate = rt::rate_from_delta(delta);
        let info = format!("{rate}\n\n{entropy}\n{delta}\n\n{time}");
        let text = Text::new(&info, Point::new(30, 30), style);
        text.draw(handle.deref_mut()).unwrap();

        handle.flush();
    }
}