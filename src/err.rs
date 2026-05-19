//! Errors that can occur in this crate, grouped by the module they came from.
use derive_more::From;
use strum::Display;

/// Represents an error that can occur while using the library crate.
#[derive(Debug, From, Display)]
#[allow(dead_code)]
pub enum Error {
    // ---- book_storage --------------------------------------------
    /// Occurs on a borrow attempt if all of the storage's books are already borroweds
    InvalidBookBorrow,

    /// Occurs on a return attempt if all of the storage's books are inside the storage
    InvalidBookReturn,

    /// Occurs when an overflow occurs when adding a copy of a book
    CopyAmountOverflow,

    /// Occurs when an overflow occurs when trying to borrow a book
    BorrowedAmountOverflow,

    // ---- library --------------------------------------------------
    /// Occurs when an overflow happens when trying to advance book id
    BookIdOverflow,

    /// Occurs when trying to add a book that already exists in the library
    AddExistingBookToLibrary,

    /// Occurs when trying to add book copies to a book that does not exist in the library
    AddBookCopiesOfNonExistingBook,
}

impl std::error::Error for Error {}
