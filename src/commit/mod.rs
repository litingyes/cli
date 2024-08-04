use std::process::{Command, exit};

use console::style;
use inquire::{InquireError, required, Select, Text};

use crate::config::get_config;

mod types;

fn handle_inquire_error(err: InquireError) {
    match err {
        InquireError::OperationCanceled | InquireError::OperationInterrupted => exit(1),
        _ => {
            println!("There was an error, please try again")
        }
    }
}

pub fn handle_commit() {
    let mut commit_message = types::CommitMessage::new();
    let config = get_config();

    let r#type = Select::new(
        "Select the type of change that you're committing: ",
        config.types,
    )
    .with_help_message("↑↓ to move, enter to select, type to filter (Ctrl-C to exit)")
    .prompt();
    match r#type {
        Ok(r#type) => {
            commit_message.set_type(r#type.key);
        }
        Err(err) => handle_inquire_error(err),
    }

    let scope = Text::new("What is the scope of this change: ")
        .with_help_message("e.g. component or file name (Ctrl-C to exit)")
        .prompt();
    match scope {
        Ok(scope) => commit_message.set_scope(scope),
        Err(err) => handle_inquire_error(err),
    }

    let subject = Text::new("Write a short, imperative tense description of the change: ")
        .with_validator(required!("Subject cannot be empty."))
        .with_help_message("Ctrl-C to exit")
        .prompt();
    match subject {
        Ok(subject) => commit_message.set_subject(subject),
        Err(err) => handle_inquire_error(err),
    }

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message.to_string())
        .status()
        .expect("Commit failed");

    if output.success() {
        println!(
            "Git commit success: {}",
            style(commit_message.to_string()).green()
        );
    } else {
        println!("Git commit failed: {}", style(output).red());
    }
}
