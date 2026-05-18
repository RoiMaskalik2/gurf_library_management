//! Errors that can occur in this crate, grouped by the module they came from.
use derive_more::From;
use strum::Display;

/// Represents an error that can occur while using the library crate.
#[derive(Debug, From, Display)]
#[allow(dead_code)]
pub enum Error {
    /// Placeholder error
    PlaceHolder,
}

impl std::error::Error for Error {}
