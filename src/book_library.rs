//! Library Book module that will export an API for the following functionalities:
//! 1. Add a new book to the library and create a distinct id for the book
//! 2. Remove book entirely from all library storage
//! 3. Show a summary of all of the books in the library
//! 4. provide access to the [`BookStorage`] struct for a specific book id

use crate::{Error, Result, book::Book, storage::Storage};
use std::{collections::HashMap, fmt};
/// This struct will provide an API to the functionality of the module
#[derive(Debug)]
pub struct BookLibrary {
    /// Contains all of the book storages of the library
    book_storages: HashMap<Book, Storage>,
}

impl BookLibrary {
    /// Creates a new empty BookLibrary instance
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// creates an empty book storage and returns an id of the book inside the library
    /// Fails when a book with identical charectaristics is already established as a book storage in the library.
    pub fn add_book(&mut self, book: Book) -> Result<()> {
        // Validate the book does not exist in the library
        if self.book_storages.contains_key(&book) {
            return Err(Error::AddExistingBookToLibrary);
        }

        // Create the book storage and add it to the library
        self.book_storages.insert(book, Storage::new());

        Ok(())
    }

    /// Entirely removes a book and all of it's copies from the library
    pub fn remove_book(&mut self, book: &Book) -> Result<()> {
        self.book_storages
            .remove(book)
            .ok_or(Error::NonExistingBook)?;

        Ok(())
    }

    /// Get a reference to a book storage of some book if it exists in the library
    pub fn get_book_storage(&self, book: &Book) -> Result<&Storage> {
        self.book_storages.get(book).ok_or(Error::NonExistingBook)
    }

    /// Get a mutable reference to a book storage of some book if it exists in the library
    pub fn get_mut_book_storage(&mut self, book: &Book) -> Result<&mut Storage> {
        self.book_storages
            .get_mut(book)
            .ok_or(Error::NonExistingBook)
    }

    /// Print information about a specific book in the library and it's storage if it exists
    pub fn print_book_information(&self, book: &Book) -> Result<()> {
        let book_storage = self.book_storages.get(book).ok_or(Error::NonExistingBook)?;

        println!("Book: {}", book);
        println!("{}\n", book_storage);

        Ok(())
    }
}

impl Default for BookLibrary {
    fn default() -> Self {
        Self {
            book_storages: HashMap::default(),
        }
    }
}

impl fmt::Display for BookLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (book, book_storage) in &self.book_storages {
            writeln!(f, "Book: {}", book)?;
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

    fn make_library_with_one_book() -> Result<BookLibrary> {
        let mut library = BookLibrary::new();
        library.add_book(make_book(BOOK_1))?;

        Ok(library)
    }

    #[test]
    fn library_creation() -> Result<()> {
        let library = BookLibrary::new();
        assert!(library.book_storages.is_empty());

        Ok(())
    }

    #[test]
    fn successful_add_book() -> Result<()> {
        let mut library = BookLibrary::new();
        library.add_book(make_book(BOOK_1))?;
        assert_eq!(library.book_storages.len(), 1);

        Ok(())
    }

    #[test]
    fn add_existing_book() -> Result<()> {
        let mut library = make_library_with_one_book()?;
        let result = library.add_book(make_book(BOOK_1));
        assert!(matches!(result, Err(Error::AddExistingBookToLibrary)));

        Ok(())
    }

    #[test]
    fn add_book_same_name_different_author() -> Result<()> {
        let mut library = make_library_with_one_book()?;
        library.add_book(make_book((BOOK_1.0, BOOK_2.1)))?;

        Ok(())
    }

    #[test]
    fn add_book_same_author_different_name() -> Result<()> {
        let mut library = make_library_with_one_book()?;
        library.add_book(make_book((BOOK_2.0, BOOK_1.1)))?;

        Ok(())
    }

    #[test]
    fn successful_remove_book() -> Result<()> {
        let mut library = make_library_with_one_book()?;
        library.remove_book(&make_book(BOOK_1))?;
        assert_eq!(library.book_storages.len(), 0);

        Ok(())
    }

    #[test]
    fn failed_remove_non_existing_book() -> Result<()> {
        let mut library = BookLibrary::new();
        let result = library.remove_book(&make_book(BOOK_1));
        assert!(matches!(result, Err(Error::NonExistingBook)));

        Ok(())
    }

    #[test]
    fn successful_get_book_storage() -> Result<()> {
        let library = make_library_with_one_book()?;
        library.get_book_storage(&make_book(BOOK_1))?;

        Ok(())
    }

    #[test]
    fn failed_get_non_existing_book_storage() -> Result<()> {
        let library = BookLibrary::new();
        let result = library.get_book_storage(&make_book(BOOK_1));
        assert!(matches!(result, Err(Error::NonExistingBook)));

        Ok(())
    }

    #[test]
    fn test_complex_add_and_remove_books() -> Result<()> {
        // Add two books
        let mut library = make_library_with_one_book()?;
        library.add_book(make_book(BOOK_2))?;

        assert_eq!(library.book_storages.len(), 2);

        // Remove a book and check the other is still accessible
        library.remove_book(&make_book(BOOK_2))?;

        assert_eq!(library.book_storages.len(), 1);
        assert!(matches!(
            library.get_book_storage(&make_book(BOOK_2)),
            Err(Error::NonExistingBook)
        ));

        // Remaining book is still accessible
        library.get_book_storage(&make_book(BOOK_1))?;

        // Add a new book and verify it is accessible
        library.add_book(make_book(BOOK_3))?;
        assert_eq!(library.book_storages.len(), 2);

        Ok(())
    }
}
