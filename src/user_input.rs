//! User input handling module.
//!
//! includes functions for  user input from the command line.

use crate::{Error, Result, UserChoice, book::Book};
use std::io;

/// Reads a string from the standard input, trims it, and checks that the input is not empty.
pub fn input_string(display_message: &str) -> Result<String> {
    println!("{}", display_message);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    user_input.truncate(user_input.trim_end().len());

    (!user_input.is_empty())
        .then_some(())
        .ok_or(Error::EmptyString)?;

    Ok(user_input)
}

/// Reads an input from the standard input, trims it
/// valates that the input is an i32 type and returns the input as an integer
pub fn input_unsigned_integer(display_message: &str) -> Result<u32> {
    let user_input = input_string(display_message)?;

    let integer_input = user_input.parse()?;
    Ok(integer_input)
}

/// Reads input from the standard input and converts it into a val [`UserChoice`]
/// In order to be able to use the input in [`crate::BookLibrary`]
pub fn read_library_user_choice() -> Result<UserChoice> {
    let operation_choice = input_unsigned_integer("Your choice: ")?;

    match operation_choice {
        1 => {
            let book = input_book()?;
            Ok(UserChoice::AddBookToLibrary(book))
        }
        2 => {
            let book = input_book()?;
            Ok(UserChoice::RemoveBookFromLibrary(book))
        }
        3 => {
            let book = input_book()?;
            let copy_amount = input_unsigned_integer("Enter number of copies to add: ")?;
            Ok(UserChoice::AddBookCopies((book, copy_amount)))
        }
        4 => {
            let book = input_book()?;
            Ok(UserChoice::BorrowBook(book))
        }
        5 => {
            let book = input_book()?;
            Ok(UserChoice::ReturnBook(book))
        }
        6 => {
            let book = input_book()?;
            Ok(UserChoice::PrintBookInformation(book))
        }
        7 => Ok(UserChoice::PrintLibraryBookInformation),
        8 => Ok(UserChoice::ExitCli),
        _ => Err(Error::InvalidCliChoice),
    }
}

/// Reads input from the stdin and converts it into a [`Book`] struct
pub fn input_book() -> Result<Book> {
    let book_name = input_string("Enter Book name: ")?;
    let book_author = input_string("Enter Book author: ")?;

    Ok(Book::new(book_name, book_author))
}
