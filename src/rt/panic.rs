use core::panic::PanicInfo;
use uefi::{boot, println, system};
use crate::{gfx, rt};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = system::with_stdout(|x| x.reset(false));
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
    println!("{info}");

    gfx::panic(info);
    
    loop {
        boot::stall(60 * rt::SECOND);
    }
}