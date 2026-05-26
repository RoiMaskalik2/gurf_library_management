use library_management::{BookLibraryCli, Result, user_input::read_library_user_choice};
use std::ops::ControlFlow;

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    run_interactive_cli()?;

    Ok(())
}

fn run_interactive_cli() -> Result<()> {
    let mut library_cli = BookLibraryCli::new();
    loop {
        println!("{}", library_cli);

        let user_choice = read_library_user_choice()?;
        if let ControlFlow::Break(_) = library_cli.perform_operation(user_choice)? {
            break;
        }
        print!("\n\n");
    }

    Ok(())
}
