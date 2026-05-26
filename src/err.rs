//! Errors that can occur in this crate, grouped by the module they came from.
use std::{io, num};

/// Represents an error that can occur while using the library crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ---- storage --------------------------------------------
    /// Occurs on a borrow attempt if all of the storage's items are already borroweds
    #[error("{self:?}")]
    InvalidItemBorrow,

    /// Occurs on a return attempt if all of the storage's items are inside the storage
    #[error("{self:?}")]
    InvalidItemReturn,

    /// Occurs when an overflow occurs when adding a copy of an item
    #[error("{self:?}")]
    CopyAmountOverflow,

    /// Occurs when an overflow occurs when trying to borrow an item
    #[error("{self:?}")]
    BorrowedAmountOverflow,

    // ---- library --------------------------------------------------
    /// Occurs when an overflow happens when trying to advance book id
    #[error("{self:?}")]
    BookIdOverflow,

    /// Occurs when trying to add a book that already exists in the library
    #[error("{self:?}")]
    AddExistingBookToLibrary,

    /// Occurs when trying to accesss a book that does not exist in the library
    #[error("{self:?}")]
    NonExistingBook,

    // ---- user_input -----------------------------------------------
    /// User provided an empty input.
    #[error("{self:?}")]
    EmptyString,

    /// Error occurred during input reading.
    #[error("{self:?}")]
    Io(#[from] io::Error),

    /// Error occurred during an input conversation to an integer.
    #[error("{self:?}")]
    ParseInt(#[from] num::ParseIntError),

    /// Error occurred during Parsing a library user choice command.
    #[error("{self:?}")]
    ParseCommand(#[from] shell_words::ParseError),

    // ---- library_cli ----------------------------------------------
    /// User Did not choose a valid cli choice
    #[error("{self:?}")]
    InvalidCliChoice(#[from] clap::Error),
}
