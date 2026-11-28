use alloc::{boxed::Box, string::String, vec::Vec};
use uefi::{CStr16, fs::UefiDirectoryIter, proto::media::file::FileInfo};
use crate::{Generalize, Result, fs::{self, Path}};

/// Copies the contents of the given [`Path`] to the other given [`Path`].
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let from = CStr16::from_str_with_buf(from.as_ref(), &mut buf)?;
    let mut buf = fs::UEFI_PATH_BUF;
    let to = CStr16::from_str_with_buf(to.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().copy(from, to).generalize()
}

/// Creates a directory at the given [`Path`].
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().create_dir(path).generalize()
}

/// Recursively creates directories until the given [`Path`] is reached.
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().create_dir_all(path).generalize()
}

/// Returns `Ok(true)` if the given [`Path`] exists, `Ok(false)` if it doesn't,
/// and `Err(...)` if neither can be confirmed.
pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().try_exists(path).generalize()
}

/// Returns information about the contents at the given [`Path`].
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Box<FileInfo>> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().metadata(path).generalize()
}

/// Returns the [`Path`]'s contents as a byte [`Vec`].
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().read(path).generalize()
}

/// Returns an iter of the contents at the given [`Path`].
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<UefiDirectoryIter> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().read_dir(path).generalize()
}

/// Returns the [`Path`]'s contents as a [`String`].
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().read_to_string(path).generalize()
}

/// Removes the directory at the given [`Path`] as long as it is empty.
pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().remove_dir(path).generalize()
}

/// Removes the directory and its contents at the given [`Path`].
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().remove_dir_all(path).generalize()
}

/// Removes the file at the given [`Path`].
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().remove_file(path).generalize()
}

/// Renames the contents at the given [`Path`] to the other given [`Path`].
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let from = CStr16::from_str_with_buf(from.as_ref(), &mut buf)?;
    let mut buf = fs::UEFI_PATH_BUF;
    let to = CStr16::from_str_with_buf(to.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().rename(from, to).generalize()
}

/// Writes to the given [`Path`] a given slice of data.
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C)
 -> Result<()> {
    let mut buf = fs::UEFI_PATH_BUF;
    let path = CStr16::from_str_with_buf(path.as_ref(), &mut buf)?;
    fs::HANDLE.get_mut().write(path, contents).generalize()
}