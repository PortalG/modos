//! The ModOS Filesystem.
//! 
//! Abstracts UEFI's `SimpleFilesystemProtocol`, defines common paths, and
//! implements encryption based reading and writing.

mod util; pub use util::*;

use uefi::{boot, fs::FileSystem};
use crate::rt::Handle;

/// Path type for specifying file or directory locations via [`str`].
pub type Path = str;

/// Defines an empty UEFI path buffer.
pub const UEFI_PATH_BUF: [u16; 256] = [0; 256];

/// UEFI filesystem handle.
pub static HANDLE: Handle<FileSystem> = Handle::none();

/// Initializes the filesystem [`HANDLE`].
/// 
/// Must be called before any file-based operations are attempted.
pub fn init() {
    let proto = boot::get_image_file_system(boot::image_handle()).unwrap();
    HANDLE.init(FileSystem::new(proto));
}