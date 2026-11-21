//! ### The ModOS Runtime
//! 
//! Manages everything related to the execution life of the operating system.

use uefi::{Result, boot, helpers};

/// Pre-configs the builtin UEFI helpers and disables the watchdog timer. 
pub fn preconfig() -> Result<()> {
    helpers::init()?;
    boot::set_watchdog_timer(0, 0, None)
}