//! User input handling module.
//!
//! includes functions for  user input from the command line.
use crate::{Error, Result, UserChoice};
use clap::Parser;
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
    let user_command = input_string("Command: ")?;

    let user_choice = UserChoice::try_parse_from(shell_words::split(&user_command)?.iter())?;

    Ok(user_choice)
}
