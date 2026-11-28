#![no_std]
extern crate alloc;

pub mod fs;
pub mod gfx;
pub mod rt;

use core::{cell::UnsafeCell, error, ops::Deref, result};
use alloc::boxed::Box;

/// Generic [`Box`]ed [`Error`](error::Error) type.
pub type Error = Box<dyn error::Error>;

/// Generic [`Result`](result::Result) type that [`Box`]es the [`Err`] result.
pub type Result<T> = result::Result<T, Error>;

/// Trait for making unique types more generic.
pub trait Generalize<T> {
    /// Makes the value into a more generic, commonly used type.
    fn generalize(self) -> T;
}

impl<E: error::Error + 'static> Generalize<Error> for E {
    fn generalize(self) -> Error {
        Box::new(self)
    }
}

impl<T, E: error::Error + 'static> Generalize<Result<T>>
for result::Result<T, E> {
    fn generalize(self) -> Result<T> {
        self.map_err(|e| e.generalize())
    }
}

/// Static wrapper type that guarantees data will be stored statically, and be
/// mutably referenceable without any extra expensive syncing operations.
pub struct Static<T>(UnsafeCell<T>);

impl<T> Static<T> {
    /// Returns a value wrapped in a [`Static`].
    pub const fn new(x: T) -> Self {
        Self(UnsafeCell::new(x))
    }

    /// Overwrites the inner value of the [`Static`] to a newly given value.
    pub fn overwrite(&self, x: T) {
        let mut_self = self.as_mut();
        *mut_self = x
    }

    /// Returns a mutable reference to the inner value.
    #[allow(clippy::mut_from_ref)]
    pub fn as_mut(&self) -> &mut T {
        // SAFETY: ModOS is single-threaded, and designed to have
        // non-overlapping execution. Data-races will not be present.
        unsafe { self.0.as_mut_unchecked() }
    }
}

impl<T> Deref for Static<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ModOS is single-threaded, and designed to have
        // non-overlapping execution. Data-races will not be present.
        unsafe { self.0.as_ref_unchecked() }
    }
}

// SAFETY: ModOS is single-threaded, and designed to have non-overlapping
// execution. Data-races will not be present.
unsafe impl<T> Sync for Static<T> {}