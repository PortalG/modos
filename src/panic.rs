use core::panic::PanicInfo;
use log::error;
use uefi::{boot, println};
use crate::rt;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(
"
    --------------------------------------------------
    MODOS PANICKED!
    RUNTIME HAS BEEN SUSPENDED.

    LOOK BELOW FOR MORE DETAILS, AND THEN HOLD POWER
    UNTIL THE SYSTEM IS COMPLETELY OFF.
    --------------------------------------------------
"
    );
    error!("{info}");
    
    loop {
        boot::stall(60 * rt::SECOND);
    }
}