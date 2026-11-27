//! ### The ModOS Runtime
//! 
//! Manages everything related to the execution life of the operating system.

mod delta; pub use delta::*;
mod init;  pub use init::*;
mod error; pub use error::*;

use uefi::{boot, helpers, println};

/// Defines the amount of microseconds in a single second.
pub const SECOND: usize = 1_000_000;

/// Defines the amount of microseconds in a millisecond.
pub const MILLI: usize = 1_000;

/// Pre-configs the builtin UEFI helpers and disables the watchdog timer. 
pub fn preconfig() -> crate::Result<()> {
    helpers::init()?;
    boot::set_watchdog_timer(0, 0, None)?;
    Ok(())
}

/// Used to pause the runtime manually. ONLY FOR DEBUGGING!
#[deprecated(note =
    "this function is only for debugging, and as such, \
    should be removed in production code."
)]
pub fn pause() -> ! {
    println!(
        "
    --------------------------------------------------
    MODOS RUNTIME HAS BEEN MANUALLY PAUSED.

    IF YOU ARE NOT A DEVELOPER AND HAVE STUMBLED UPON
    THIS, THAT MEANS I... LEWIS JAYDEN BOYCE...
    
    AM A DUMBASS.

    SINCERELY, PORTAL.
    --------------------------------------------------
        "
    );

    loop {
        boot::stall(60 * SECOND);
    }
}