//! Library Management
//!
//! Exports the building blocks for interacting with book and book library

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod book;
pub mod book_library;
mod err;
pub mod library_cli;
pub mod storage;
pub mod user_input;

pub use book_library::BookLibrary;
pub use library_cli::{BookLibraryCli, UserChoice};

pub use err::Error;
/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
