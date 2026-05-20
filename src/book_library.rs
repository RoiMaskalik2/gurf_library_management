//! Library Book module that will export an API for the following functionalities:
//! 1. Add a new book to the library and create a distinct id for the book
//! 2. Remove book entirely from all library storage
//! 3. Show a summary of all of the books in the library
//! 4. provide access to the [`BookStorage`] struct for a specific book id

use crate::{Error, Result, book::Book, book_storage::BookStorage};
use std::{collections::HashMap, fmt};

// Start from the lowest possible book id so that the library will have as many id to allocate as possible
const STARTING_BOOK_ID: u32 = 0;

// The book id logic will be to increment the previous given id on each new book that is added to the library
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

    /// Entirely removes a book and all of it's copies from the library
    pub fn remove_book(&mut self, book_id: u32) -> Result<()> {
        self.book_storages
            .remove(&book_id)
            .ok_or(Error::NonExistingBookId)?;

        Ok(())
    }

    /// Get a reference to a book storage of some book if it exists in the library
    pub fn get_book_storage(&self, book_id: u32) -> Result<&BookStorage> {
        self.book_storages
            .get(&book_id)
            .ok_or(Error::NonExistingBookId)
    }

    /// Get a mutable reference to a book storage of some book if it exists in the library
    pub fn get_mut_book_storage(&mut self, book_id: u32) -> Result<&mut BookStorage> {
        self.book_storages
            .get_mut(&book_id)
            .ok_or(Error::NonExistingBookId)
    }
}

impl Default for BookLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BookLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (&book_id, book_storage) in &self.book_storages {
            writeln!(f, "--------------ID: {} -------------------", book_id)?;
            writeln!(f, "{}\n", book_storage)?;
        }

        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const BOOK_1: (&str, &str) = ("How To Make Maskalik Rashatz", "Gurfpovski");
    const BOOK_2: (&str, &str) = (
        "What 80% Of your role as rashatz will be",
        "Gurfilious The III Jr.",
    );
    const BOOK_3: (&str, &str) = (
        "Your Pr does not count until Gurfinstein approves",
        "Gurfinstein",
    );

    fn make_book((name, author): (&str, &str)) -> Book {
        Book::new(name.to_string(), author.to_string())
    }

    fn make_library_with_one_book() -> Result<(BookLibrary, u32)> {
        let mut library = BookLibrary::new();
        let id = library.add_book(make_book(BOOK_1))?;

        Ok((library, id))
    }

    #[test]
    fn library_creation() -> Result<()> {
        let library = BookLibrary::new();
        assert!(library.book_storages.is_empty());
        assert_eq!(library.next_id, STARTING_BOOK_ID);

        Ok(())
    }

    #[test]
    fn successful_add_book() -> Result<()> {
        let mut library = BookLibrary::new();
        let id = library.add_book(make_book(BOOK_1))?;
        assert_eq!(id, STARTING_BOOK_ID);
        assert_eq!(library.book_storages.len(), 1);

        Ok(())
    }

    #[test]
    fn add_existing_book() -> Result<()> {
        let (mut library, _) = make_library_with_one_book()?;
        let result = library.add_book(make_book(BOOK_1));
        assert!(matches!(result, Err(Error::AddExistingBookToLibrary)));

        Ok(())
    }

    #[test]
    fn add_book_same_name_different_author() -> Result<()> {
        let (mut library, _) = make_library_with_one_book()?;
        library.add_book(make_book((BOOK_1.0, BOOK_2.1)))?;

        Ok(())
    }

    #[test]
    fn add_book_same_author_different_name() -> Result<()> {
        let (mut library, _) = make_library_with_one_book()?;
        library.add_book(make_book((BOOK_2.0, BOOK_1.1)))?;

        Ok(())
    }

    #[test]
    fn successful_remove_book() -> Result<()> {
        let (mut library, id) = make_library_with_one_book()?;
        library.remove_book(id)?;
        assert_eq!(library.book_storages.len(), 0);

        Ok(())
    }

    #[test]
    fn failed_remove_non_existing_book() -> Result<()> {
        let mut library = BookLibrary::new();
        let result = library.remove_book(100);
        assert!(matches!(result, Err(Error::NonExistingBookId)));

        Ok(())
    }

    #[test]
    fn successful_get_book_storage() -> Result<()> {
        let (library, id) = make_library_with_one_book()?;
        let storage = library.get_book_storage(id)?;
        assert!(storage.get_book() == &make_book(BOOK_1));

        Ok(())
    }

    #[test]
    fn failed_get_non_existing_book_storage() -> Result<()> {
        let library = BookLibrary::new();
        let result = library.get_book_storage(100);
        assert!(matches!(result, Err(Error::NonExistingBookId)));

        Ok(())
    }

    #[test]
    fn test_complex_add_and_remove_books() -> Result<()> {
        // Add two books
        let (mut library, id1) = make_library_with_one_book()?;
        let id2 = library.add_book(make_book(BOOK_2))?;

        assert_eq!(library.book_storages.len(), 2);

        // Remove a book and check the other is still accessible
        library.remove_book(id2)?;

        assert_eq!(library.book_storages.len(), 1);
        assert!(matches!(
            library.get_book_storage(id2),
            Err(Error::NonExistingBookId)
        ));

        // Remaining book is still accessible
        library.get_book_storage(id1)?;

        // Add a new book and make sure it gets a new id that did not exist
        let id3 = library.add_book(make_book(BOOK_3))?;
        assert_ne!(id3, id1);
        assert_ne!(id3, id2);
        assert_eq!(library.book_storages.len(), 2);

        Ok(())
    }
}
