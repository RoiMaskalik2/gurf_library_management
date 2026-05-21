//! This modules implements an interactive API to use the [`BookLibrary`] struct.

use crate::{BookLibrary, Result, book::Book};
use std::{fmt, ops::ControlFlow};
use strum::{EnumIter, IntoEnumIterator};

/// This enum represents all of the interactive user choices with the [`BookLibrary`] struct.
#[derive(EnumIter)]
pub enum UserChoice {
    /// Adds a book to the library
    AddBookToLibrary(Book),

    /// Removes a book and all of it's copies from the library
    RemoveBookFromLibrary(u32),

    /// Adds copies to an existing book in the library
    AddBookCopies((u32, u32)),

    /// Borrow a specific book id if it exists in the library and there are available copies to borrow
    BorrowBook(u32),

    /// Return a borrowed book to the library
    ReturnBook(u32),

    /// Print information about a specific book
    PrintBookInformation(u32),

    /// Print information about the entire books in the library
    PrintLibraryBookInformation,

    /// Stop running the interactive cli
    ExitCli,
}

impl fmt::Display for UserChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserChoice::AddBookToLibrary(_) => writeln!(f, "1) Add a book to the library"),
            UserChoice::RemoveBookFromLibrary(_) => {
                writeln!(f, "2) Remove a book from the library")
            }
            UserChoice::AddBookCopies(_) => writeln!(f, "3) add copies of an existing book"),
            UserChoice::BorrowBook(_) => writeln!(f, "4) Borrow a book"),
            UserChoice::ReturnBook(_) => writeln!(f, "5) Return a borrowed book"),
            UserChoice::PrintBookInformation(_) => {
                writeln!(f, "6) Print information for a specific book")
            }
            UserChoice::PrintLibraryBookInformation => {
                writeln!(f, "7) Print information for all books")
            }
            UserChoice::ExitCli => writeln!(f, "8) Exit CLI"),
        }
    }
}

/// This struct will provide the api for a user to interact with the [`BookLibrary`] struct
#[derive(Default)]
pub struct BookLibraryCli {
    /// Book library
    book_library: BookLibrary,
}

impl BookLibraryCli {
    /// Construct a new cli struct that will contain book library information
    pub fn new() -> Self {
        Self {
            book_library: BookLibrary::default(),
        }
    }

    /// Receive a user choice and perform the appropriate operation on the interactice library
    pub fn perform_operation(&mut self, user_choice: UserChoice) -> Result<ControlFlow<()>> {
        match user_choice {
            UserChoice::AddBookToLibrary(book) => self.add_book_to_library(book),
            UserChoice::RemoveBookFromLibrary(book_id) => self.remove_book_from_library(book_id),
            UserChoice::AddBookCopies((book_id, copy_amount)) => {
                self.add_book_copies(book_id, copy_amount)
            }
            UserChoice::BorrowBook(book_id) => self.borrow_book(book_id),
            UserChoice::ReturnBook(book_id) => self.return_book(book_id),
            UserChoice::PrintBookInformation(book_id) => self.print_book(book_id),
            UserChoice::PrintLibraryBookInformation => self.print_library(),
            UserChoice::ExitCli => Ok(ControlFlow::Break(())),
        }
    }

    fn add_book_to_library(&mut self, book: Book) -> Result<ControlFlow<()>> {
        self.book_library.add_book(book)?;

        Ok(ControlFlow::Continue(()))
    }

    fn remove_book_from_library(&mut self, book_id: u32) -> Result<ControlFlow<()>> {
        self.book_library.remove_book(book_id)?;

        Ok(ControlFlow::Continue(()))
    }

    fn add_book_copies(&mut self, book_id: u32, copy_amount: u32) -> Result<ControlFlow<()>> {
        self.book_library
            .get_mut_book_storage(book_id)?
            .add_book_copies(copy_amount)?;

        Ok(ControlFlow::Continue(()))
    }

    fn borrow_book(&mut self, book_id: u32) -> Result<ControlFlow<()>> {
        self.book_library
            .get_mut_book_storage(book_id)?
            .borrow_book()?;

        Ok(ControlFlow::Continue(()))
    }

    fn return_book(&mut self, book_id: u32) -> Result<ControlFlow<()>> {
        self.book_library
            .get_mut_book_storage(book_id)?
            .return_book()?;

        Ok(ControlFlow::Continue(()))
    }

    fn print_book(&self, book_id: u32) -> Result<ControlFlow<()>> {
        println!(
            "{}",
            self.book_library.get_book_storage(book_id)?.get_book()
        );

        Ok(ControlFlow::Continue(()))
    }

    fn print_library(&self) -> Result<ControlFlow<()>> {
        print!("{}", self.book_library);

        Ok(ControlFlow::Continue(()))
    }
}

impl fmt::Display for BookLibraryCli {
    /// Displays the full CLI menu.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Library Options:")?;

        for choice in UserChoice::iter() {
            write!(f, "  {choice}")?;
        }
        Ok(())
    }
}
