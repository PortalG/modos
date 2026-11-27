use core::{error, fmt::Display, result};
use alloc::{borrow::ToOwned, boxed::Box, string::String};

/// Error type that contains a message and an optionally given identifier.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    /// Provided message.
    pub message: String,
    /// Optionally provided identifier.
    pub id: usize
}

impl Error {
    /// Creates a new [`Error`] from a given message. The identifier defaults to
    /// `1`.
    pub fn new(message: &str) -> Self {
        let message = message.to_owned();
        Self { message, id: 1 }
    }

    /// Creates a new [`Error`] from a given message and identifier.
    pub fn new_with_id(message: &str, id: usize) -> Self {
        let message = message.to_owned();
        Self { message, id }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{} ({})", self.message, self.id)
    }
}

impl error::Error for Error {}

/// [`Error`](crate::Error) conversion trait.
pub trait IntoError {
    /// Converts a [`str`] into an [`Error`](crate::Error).
    fn into_error(self) -> crate::Error;

    /// Converts a [`str`] into an [`Error`](crate::Error) with a given
    /// identifier.
    fn into_error_with_id(self, id: usize) -> crate::Error;

    /// Converts a [`str`] into a simple runtime [`Error`].
    fn into_runtime_error(self) -> Error;
    
    /// Converts a [`str`] into a simple runtime [`Error`] with a given
    /// identifier.
    fn into_runtime_error_with_id(self, id: usize) -> Error;
}

impl IntoError for &str {
    fn into_error(self) -> crate::Error {
        Box::new(Error::new(self))
    }
    
    fn into_error_with_id(self, id: usize) -> crate::Error {
        Box::new(Error::new_with_id(self, id))
    }
    
    fn into_runtime_error(self) -> Error {
        Error::new(self)
    }
    
    fn into_runtime_error_with_id(self, id: usize) -> Error {
        Error::new_with_id(self, id)
    }
}

/// Result type that can only result in a simple runtime [`Error`].
pub type Result<T> = result::Result<T, Error>;