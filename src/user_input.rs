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
/// validates that the input is an i32 type and returns the input as an integer
pub fn input_unsigned_integer(display_message: &str) -> Result<u32> {
    let user_input = input_string(display_message)?;

    let integer_input = user_input.parse()?;
    Ok(integer_input)
}

/// Reads input from the standard input and converts it into a valid [`UserChoice`]
/// In order to be able to use the input in [`crate::BookLibrary`]
pub fn read_library_user_choice() -> Result<UserChoice> {
    let operation_choice = input_unsigned_integer("Your choice: ")?;

    match operation_choice {
        1 => {
            let book_name = input_string("Enter book name: ")?;
            let book_author = input_string("Enter book author: ")?;
            Ok(UserChoice::AddBookToLibrary(Book::new(
                book_name,
                book_author,
            )))
        }
        2 => {
            let book_id = input_unsigned_integer("Enter book ID to remove: ")?;
            Ok(UserChoice::RemoveBookFromLibrary(book_id))
        }
        3 => {
            let book_id = input_unsigned_integer("Enter book ID to add copies to: ")?;
            let copy_amount = input_unsigned_integer("Enter number of copies to add: ")?;
            Ok(UserChoice::AddBookCopies((book_id, copy_amount)))
        }
        4 => {
            let book_id = input_unsigned_integer("Enter book ID to borrow: ")?;
            Ok(UserChoice::BorrowBook(book_id))
        }
        5 => {
            let book_id = input_unsigned_integer("Enter book ID to return: ")?;
            Ok(UserChoice::ReturnBook(book_id))
        }
        6 => {
            let book_id = input_unsigned_integer("Enter book ID to print information about: ")?;
            Ok(UserChoice::PrintBookInformation(book_id))
        }
        7 => Ok(UserChoice::PrintLibraryBookInformation),
        8 => Ok(UserChoice::ExitCli),
        _ => Err(Error::InvalidCliChoice),
    }
}
