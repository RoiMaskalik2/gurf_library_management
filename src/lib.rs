//! Library Management
//!
//! Exports the building blocks for interacting with book and book library

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod book;
pub mod book_library;
pub mod book_storage;
mod err;

pub use err::Error;
/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
