use core::ops::{Deref, DerefMut};
use uefi::{
    Handle, boot::{self, ScopedProtocol},
    proto::console::gop::{GraphicsOutput, ModeInfo}
};
use uefi_graphics2::UefiDisplay;

/// UEFI display type that bundles [`UefiDisplay`], [`Handle`],
/// [`ScopedProtocol<GraphicsOutput>`], and [`ModeInfo`] into one.
pub struct Display(
    UefiDisplay, Handle, ScopedProtocol<GraphicsOutput>, ModeInfo
);

impl Display {
    /// Returns a new [`Display`].
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let handle = boot::get_handle_for_protocol::<GraphicsOutput>()
            .unwrap();
        let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(handle)
            .unwrap();

        let mode = gop.current_mode_info();
        let display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();

        Self(display, handle, gop, mode)
    }

    /// Returns the inner [`UefiDisplay`].
    pub fn display(&mut self) -> &mut UefiDisplay {
        &mut self.0
    }

    /// Returns the inner [`Handle`].
    pub fn handle(&self) -> &Handle {
        &self.1
    }

    /// Returns the inner [`ScopedProtocol<GraphicsOutput>`].
    pub fn gop(&mut self) -> &mut ScopedProtocol<GraphicsOutput> {
        &mut self.2
    }

    /// Returns the inner [`ModeInfo`].
    pub fn mode(&self) -> &ModeInfo {
        &self.3
    }
}

impl Deref for Display {
    type Target = UefiDisplay;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}