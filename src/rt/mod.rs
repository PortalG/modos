//! The ModOS Runtime.
//! 
//! Manages everything related to the execution life of the operating system.

mod delta; pub use delta::*;
mod init;  pub use init::*;
mod panic;

use uefi::{boot, helpers, println};

/// Global handle type that can be initialized as [`None`] at compile-time, and
/// [`Some`] at runtime.
pub type Handle<T> = crate::Static<Option<T>>;

impl<T> Handle<T> {
    /// Returns a temporally invalid [`Handle`] for compile-time initialization. 
    pub const fn none() -> Self {
        Handle::new(None)
    }

    /// Initializes the [`Handle`] to the given value.
    pub fn init(&self, x: T) {
        self.overwrite(Some(x));
    }

    /// Returns a reference to the [`Handle`].
    pub fn get(&self) -> &T {
        self.as_ref()
            .expect("handle is invalid or has not yet been initialized!")
    }

    /// Returns a mutable reference to the [`Handle`].
    pub fn get_mut(&self) -> &mut T {
        self.as_mut().as_mut()
            .expect("handle is invalid or has not yet been initialized!")
    }
}

/// Defines the amount of microseconds in a second.
pub const SECOND: usize = 1_000_000;

/// Defines the amount of microseconds in a millisecond.
pub const MILLI: usize = 1_000;

/// Pre-configs the builtin UEFI helpers and disables the watchdog timer. 
pub fn preconfig() -> crate::Result<()> {
    helpers::init()?;
    boot::set_watchdog_timer(0, 0, None)?;
    Ok(())
}

/// Used to pause the runtime manually.
/// 
/// ONLY FOR DEBUGGING!
#[deprecated(note = "this function is only for debugging, and as such, should \
be removed in all prod.")]
pub fn pause() -> ! {
    println!(
"
    --------------------------------------------------
    MODOS RUNTIME HAS BEEN MANUALLY PAUSED.

    IF YOU ARE NOT A DEVELOPER AND HAVE STUMBLED UPON
    THIS, THAT MEANS I... LEWIS JAYDEN BOYCE...
    
    AM A DUMBASS.

    SINCERELY, A DUMBASS.
    --------------------------------------------------
"
    );

    loop {
        boot::stall(60 * SECOND);
    }
}