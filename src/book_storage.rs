//! Book Storage module that will provide the functionality of storing multiple copies of the same [`crate::book::Book`]
//! and providing an API to the following functionality:
//! 1. Add a copy of the book to the storage
//! 2. Borrow a book from the storage.
//! 3. Return a book to the storage.
use crate::{Error, Result, book::Book};
use std::fmt;

// Represents that all of the books in the book storage are currently inside it.
const NO_BOOKS_BORROWED: u32 = 0;

// Amount of copies a new book storage starts with
const INITIAL_BOOK_COPIES: u32 = 1;

// the amount of books to add to the borrowed amount when borrowing a book
const BORROW_BOOK: u32 = 1;

/// Provides the functionality of storing multiple copies of the same book
pub struct BookStorage {
    /// contains book information about the book that is stored in the storage
    book: Book,

    /// The total amount of copies in the book storage
    copy_amount: u32,

    /// The amount of books that were borrowed from the storage
    borrowed_amount: u32,
}

impl BookStorage {
    /// Creates a new BookStorage instance with one book copy and no borrowed books
    pub fn new(book: Book) -> Self {
        Self {
            book,
            copy_amount: INITIAL_BOOK_COPIES,
            borrowed_amount: NO_BOOKS_BORROWED,
        }
    }

    /// Allows to borrow a book from the storage if it is available to borrow.
    /// A book is available to borrow if not all of the copies of the book from the storage were borrowed at that time.
    pub fn borrow_book(&mut self) -> Result<()> {
        if self.borrowed_amount == self.copy_amount {
            return Err(Error::InvalidBookBorrow);
        }

        self.borrowed_amount = self
            .borrowed_amount
            .checked_add(BORROW_BOOK)
            .ok_or(Error::BorrowedAmountOverflow)?;

        Ok(())
    }

    /// Allows to return a book to the storage if at least one book was borrowed at that time
    pub fn return_book(&mut self) -> Result<()> {
        self.borrowed_amount = self
            .borrowed_amount
            .checked_sub(BORROW_BOOK)
            .ok_or(Error::InvalidBookReturn)?;

        Ok(())
    }

    /// Add book copies to the storage
    pub fn add_book_copies(&mut self, amount_to_add: u32) -> Result<()> {
        self.copy_amount = self
            .copy_amount
            .checked_add(amount_to_add)
            .ok_or(Error::CopyAmountOverflow)?;

        Ok(())
    }

    /// Getter for the book in the storage
    pub fn get_book(&self) -> &Book {
        &self.book
    }
}

impl fmt::Display for BookStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nCopy Amount: {}\nBooks Borrowed: {}",
            self.book, self.copy_amount, self.borrowed_amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_storage() -> BookStorage {
        BookStorage::new(Book::new(
            "Teaching Maskalik to be a hedgehog".to_string(),
            "Sir Gurfinson".to_string(),
        ))
    }

    #[test]
    fn storage_creaiton() -> Result<()> {
        let storage = make_storage();
        assert_eq!(storage.copy_amount, INITIAL_BOOK_COPIES);
        assert_eq!(storage.borrowed_amount, NO_BOOKS_BORROWED);

        Ok(())
    }

    #[test]
    fn successful_book_borrow() -> Result<()> {
        let mut storage = make_storage();
        storage.borrow_book()?;
        assert_eq!(storage.borrowed_amount, 1);

        Ok(())
    }

    #[test]
    fn borrow_unavailable_book() -> Result<()> {
        let mut storage = make_storage();
        storage.borrow_book()?;
        let result = storage.borrow_book();
        assert!(matches!(result, Err(Error::InvalidBookBorrow)));

        Ok(())
    }

    #[test]
    fn successful_return_after_borrow() -> Result<()> {
        let mut storage = make_storage();
        storage.borrow_book()?;
        storage.return_book()?;
        assert_eq!(storage.borrowed_amount, NO_BOOKS_BORROWED);

        Ok(())
    }

    #[test]
    fn failed_return_when_no_books_borrowed() -> Result<()> {
        let mut storage = make_storage();
        let result = storage.return_book();
        assert!(matches!(result, Err(Error::InvalidBookReturn)));

        Ok(())
    }

    #[test]
    fn successful_add_copies() -> Result<()> {
        let mut storage = make_storage();
        storage.add_book_copies(5)?;
        assert_eq!(storage.copy_amount, 6);
        assert_eq!(storage.borrowed_amount, 0);

        Ok(())
    }

    #[test]
    fn test_complex_borrows_returns_and_add_copies() -> Result<()> {
        let mut storage = make_storage();

        // Add copies
        storage.add_book_copies(5)?;
        assert_eq!(storage.copy_amount, 6);
        assert_eq!(storage.borrowed_amount, 0);

        // Borrow some books
        storage.borrow_book()?;
        storage.borrow_book()?;
        storage.borrow_book()?;
        assert_eq!(storage.copy_amount, 6);
        assert_eq!(storage.borrowed_amount, 3);

        // Add more copies
        storage.add_book_copies(5)?;
        assert_eq!(storage.copy_amount, 11);
        assert_eq!(storage.borrowed_amount, 3);

        // Return Books
        storage.return_book()?;
        storage.return_book()?;
        storage.return_book()?;
        assert_eq!(storage.copy_amount, 11);
        assert_eq!(storage.borrowed_amount, 0);

        Ok(())
    }
}
