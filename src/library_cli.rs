//! This modules implements an interactive API to use the [`BookLibrary`] struct through CLI commands.

use crate::{BookLibrary, Result, book::Book};
use clap::{Args, CommandFactory, Parser};
use std::ops::ControlFlow;

/// This struct represents the arguments from the cli of a book
#[derive(Args)]
pub struct BookArguments {
    /// Book name
    name: String,
    /// Book author
    author: String,
}

impl From<BookArguments> for Book {
    fn from(arguments: BookArguments) -> Self {
        Book::new(arguments.name, arguments.author)
    }
}

/// This enum represents all of the interactive user choices with the cli that interact with [`BookLibrary`] struct.
#[derive(Parser)]
#[command(no_binary_name = true)]
pub enum UserChoice {
    /// <book_name> <book_author> Adds a book to the library
    Add {
        /// Book to add
        #[command(flatten)]
        book: BookArguments,
    },

    /// Removes a book and all of it's copies from the library
    Remove {
        /// Book to remove
        #[command(flatten)]
        book: BookArguments,
    },

    /// Adds copies to an existing book in the library
    AddCopies {
        /// Book to add copies to
        #[command(flatten)]
        book: BookArguments,
        /// Number of copies to add
        copy_amount: u32,
    },

    /// Borrow a specific book id if it exists in the library and there are available copies to borrow
    Borrow {
        /// Book to borrow
        #[command(flatten)]
        book: BookArguments,
    },

    /// Return a borrowed book to the library
    Return {
        /// Book to return
        #[command(flatten)]
        book: BookArguments,
    },

    /// Print information about a specific book
    PrintBook {
        /// Book to print information about
        #[command(flatten)]
        book: BookArguments,
    },

    /// Print information about the entire books in the library
    PrintLibrary,

    /// Stop running the interactive cli
    Exit,
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
            ..Default::default()
        }
    }

    /// Receive a user choice and perform the appropriate operation on the interactice library
    pub fn perform_operation(&mut self, user_choice: UserChoice) -> Result<ControlFlow<()>> {
        match user_choice {
            UserChoice::Add { book } => self.add_book_to_library(book.into()),
            UserChoice::Remove { book } => self.remove_book_from_library(&book.into()),
            UserChoice::AddCopies { book, copy_amount } => {
                self.add_book_copies(&book.into(), copy_amount)
            }
            UserChoice::Borrow { book } => self.borrow_book(&book.into()),
            UserChoice::Return { book } => self.return_book(&book.into()),
            UserChoice::PrintBook { book } => self.print_book(&book.into()),
            UserChoice::PrintLibrary => self.print_library(),
            UserChoice::Exit => Ok(ControlFlow::Break(())),
        }
    }

    fn add_book_to_library(&mut self, book: Book) -> Result<ControlFlow<()>> {
        self.book_library.add_book(book)?;

        Ok(ControlFlow::Continue(()))
    }

    fn remove_book_from_library(&mut self, book: &Book) -> Result<ControlFlow<()>> {
        self.book_library.remove_book(book)?;

        Ok(ControlFlow::Continue(()))
    }

    fn add_book_copies(&mut self, book: &Book, copy_amount: u32) -> Result<ControlFlow<()>> {
        self.book_library
            .get_mut_book_storage(book)?
            .add_copies(copy_amount)?;

        Ok(ControlFlow::Continue(()))
    }

    fn borrow_book(&mut self, book: &Book) -> Result<ControlFlow<()>> {
        self.book_library.get_mut_book_storage(book)?.borrow()?;

        Ok(ControlFlow::Continue(()))
    }

    fn return_book(&mut self, book: &Book) -> Result<ControlFlow<()>> {
        self.book_library
            .get_mut_book_storage(book)?
            .return_item()?;

        Ok(ControlFlow::Continue(()))
    }

    fn print_book(&self, book: &Book) -> Result<ControlFlow<()>> {
        self.book_library.print_book_information(book)?;

        Ok(ControlFlow::Continue(()))
    }

    fn print_library(&self) -> Result<ControlFlow<()>> {
        println!("{}", self.book_library);

        Ok(ControlFlow::Continue(()))
    }
}

impl std::fmt::Display for BookLibraryCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Available commands:")?;
        for command in UserChoice::command().get_subcommands_mut() {
            let command_usage = command.render_usage().to_string();

            let command_documentation = if let Some(about) = command.get_about() {
                about.to_string()
            } else {
                String::new()
            };

            writeln!(f, "  {:<50} {}", command_usage, command_documentation)?;
        }

        Ok(())
    }
}
