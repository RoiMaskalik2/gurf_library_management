//! Book module that will export an API for the following functionalities:
//! 1. Creating a book.
//! 2. Show a summary of a book
use std::fmt;

/// A book struct
#[derive(core::cmp::PartialEq)]
pub struct Book {
    /// Name of the book
    book_name: String,

    /// Name of the author of the book
    book_author: String,
}

impl Book {
    /// Creates a new Book instance
    pub fn new(book_name: String, book_author: String) -> Self {
        Self {
            book_author,
            book_name,
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} By {}", self.book_name, self.book_author)
    }
}
