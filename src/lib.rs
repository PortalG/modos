#![no_std]
extern crate alloc;

pub mod rt;
pub mod panic;

use core::{error, result};
use alloc::boxed::Box;

/// Generic [`Box`]ed [`Error`](error::Error) type.
pub type Error = Box<dyn error::Error>;

/// Generic [`Result`](result::Result) type that [`Box`]es the [`Err`] result.
pub type Result<T> = result::Result<T, Error>;