mod types;

use crate::config::get_config;
use console::style;
use inquire::{required, InquireError, Select, Text};
use std::process::{exit, Command};

pub fn handle_commit() {
    let mut commit_message = types::CommitMessage::new();
    let config = get_config();

    let types = config
        .types
        .iter()
        .map(|item| item.key.to_string())
        .collect();
    let r#type = Select::new("Select the type of change that you're committing: ", types)
        .with_help_message("↑↓ to move, enter to select, type to filter (control + D to exit)")
        .prompt();
    match r#type {
        Ok(r#type) => {
            commit_message.set_type(r#type.to_string());
        }
        Err(err) => match err {
            InquireError::OperationCanceled => {
                println!("{}", style("Commit canceled").yellow());
                exit(1)
            }
            _ => {
                println!("There was an error, please try again")
            }
        },
    }

    let scope = Text::new("What is the scope of this change: ")
        .with_help_message("e.g. component or file name (control + D to exit)")
        .prompt();
    match scope {
        Ok(scope) => commit_message.set_scope(scope),
        Err(err) => match err {
            InquireError::OperationCanceled => {
                println!("{}", style("Commit canceled").yellow());
                exit(1)
            }
            _ => {
                println!("There was an error, please try again")
            }
        },
    }

    let subject = Text::new("Write a short, imperative tense description of the change: ")
        .with_validator(required!("Subject cannot be empty."))
        .with_help_message("control + D to exit")
        .prompt();
    match subject {
        Ok(subject) => commit_message.set_subject(subject),
        Err(err) => match err {
            InquireError::OperationCanceled => {
                println!("{}", style("Commit canceled").yellow());
                exit(1)
            }
            _ => {
                println!("There was an error, please try again")
            }
        },
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
