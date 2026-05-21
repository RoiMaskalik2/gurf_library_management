//! Errors that can occur in this crate, grouped by the module they came from.
use std::{io, num};

/// Represents an error that can occur while using the library crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ---- book_storage --------------------------------------------
    /// Occurs on a borrow attempt if all of the storage's books are already borroweds
    #[error("{self:?}")]
    InvalidBookBorrow,

    /// Occurs on a return attempt if all of the storage's books are inside the storage
    #[error("{self:?}")]
    InvalidBookReturn,

    /// Occurs when an overflow occurs when adding a copy of a book
    #[error("{self:?}")]
    CopyAmountOverflow,

    /// Occurs when an overflow occurs when trying to borrow a book
    #[error("{self:?}")]
    BorrowedAmountOverflow,

    // ---- library --------------------------------------------------
    /// Occurs when an overflow happens when trying to advance book id
    #[error("{self:?}")]
    BookIdOverflow,

    /// Occurs when trying to add a book that already exists in the library
    #[error("{self:?}")]
    AddExistingBookToLibrary,

    /// Occurs when trying to a book that does not exist in the library
    #[error("{self:?}")]
    NonExistingBookId,

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

    // ---- library_cli ----------------------------------------------
    /// User Did not choose a valid cli choice
    #[error("{self:?}")]
    InvalidCliChoice,
}
