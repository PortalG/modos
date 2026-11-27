use log::info;
use uefi::println;
use crate::rt;

pub fn init() -> ! {
    info!("calculating cps...");
    let cps = rt::calculate_cps();
    info!("CPU sits at {} cycles per second.", cps);

    let mut time = 0.0;

    let target = 60;
    let mut entropy = 0.0;

    let mut last = rt::cycle_count();

    loop {
        let delta = rt::calculate_delta(&mut last, cps);
        rt::stall(&mut entropy, target, delta);

        time += delta;
        println!("{time}")
    }
}