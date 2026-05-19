//! Library Book module that will export an API for the following functionalities:
//! 1. Add a new book to the library and create a distinct id for the book
//! 2. Remove book entirely from all library storage
//! 3. Show a summary of all of the books in the library
//! 4. do all of the functionality of [`BookStorage`] for each of the storages that are included in the library

use std::collections::HashMap;

use crate::{
    Error, Result,
    book::Book,
    book_storage::{self, BookStorage},
};

const STARTING_BOOK_ID: u32 = 0;
const BOOK_ID_INCREMENT: u32 = 1;

/// This struct will provide an API to the functionality of the module
pub struct BookLibrary {
    /// Contains all of the book storages of the library
    book_storages: HashMap<u32, BookStorage>,

    /// An incrementing distinct counter when a new book is added to the library
    next_id: u32,
}

impl BookLibrary {
    /// Creates a new empty BookLibrary instance
    pub fn new() -> Self {
        Self {
            book_storages: HashMap::new(),
            next_id: STARTING_BOOK_ID,
        }
    }

    /// creates an empty book storage and returns an id of the book inside the library
    /// Fails when a book with identical charectaristics is already established as a book storage in the library.
    pub fn add_book(&mut self, book: Book) -> Result<u32> {
        // Validate the book does not exist in the library
        if self
            .book_storages
            .values()
            .into_iter()
            .any(|storage| storage.get_book().eq(&book))
        {
            return Err(Error::AddExistingBookToLibrary);
        }

        // Create the book storage and add it to the library
        let new_book_id = self.next_id;
        let new_book_storage = BookStorage::new(book);
        self.book_storages.insert(new_book_id, new_book_storage);

        // Increment the next id
        self.next_id = self
            .next_id
            .checked_add(BOOK_ID_INCREMENT)
            .ok_or(Error::BookIdOverflow)?;

        Ok(new_book_id)
    }

    pub fn add_book_copies(&mut self, book_id: u32, amount_to_add: u32) -> Result<()> {
        let book_storage = self
            .book_storages
            .get(&book_id)
            .ok_or(Error::AddBookCopiesOfNonExistingBook)?;
        Ok(())
    }
}
