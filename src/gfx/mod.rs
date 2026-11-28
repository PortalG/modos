//! The ModOS Graphics Engine.
//! 
//! Abstracts UEFI's `GraphicsOutputProtocol`, creates a transformable layer
//! over the frame buffer, and provides scaling and positioning abstractions.

mod gop;   pub use gop::*;
mod panic; pub use panic::*;

use log::info;
use crate::rt::Handle;

/// UEFI display handle.
pub static HANDLE: Handle<Display> = Handle::none();

/// Initializes the display [`HANDLE`].
/// 
/// Must be called before any graphics-based operations are attempted.
pub fn init() {
    info!("switching into graphics mode...");
    HANDLE.init(Display::new());
}

/// Returns if the graphics engine is running.
pub fn running() -> bool {
    HANDLE.is_some()
}