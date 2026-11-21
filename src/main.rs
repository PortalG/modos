#![no_std]
#![no_main]

use log::info;
use modos::rt;
use uefi::{Status, boot, entry};

#[entry]
fn main() -> Status {
    if rt::preconfig().is_ok() {
        info!("pre-configuration complete; starting runtime...");
        
    }
    info!("hello bob! :D");
    boot::stall(10 * 1_000_000);
    Status::SUCCESS
}