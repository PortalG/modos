#![no_std]
#![no_main]

use log::{error, info, warn};
use uefi::{Status, boot, entry};

use modos::rt;

#[entry]
fn main() -> Status {
    if let Err(e) = rt::preconfig() {
        error!("{e}");
        warn!(
            "PRE-CONFIGURATION FAILED; THINGS MIGHT BREAK; \
            STARTING RUNTIME IN 5 SECONDS..."
        );
        boot::stall(5 * rt::SECOND);
    } else {
        info!("pre-configuration complete; starting runtime...")
    }

    rt::init()
}