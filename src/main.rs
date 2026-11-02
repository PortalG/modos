#![no_std]
#![no_main]

use log::info;
use uefi::{Status, boot, entry, helpers};

#[entry]
fn main() -> Status {
    helpers::init().unwrap();
    boot::set_watchdog_timer(0, 0, None).unwrap();
    info!("hello world! :D");
    boot::stall(10 * 1_000_000);
    Status::SUCCESS
}