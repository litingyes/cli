use console::style;
use inquire::{required, Select, Text};
use std::fmt::{Display, Formatter};
use std::process::Command;

struct CommitMessage {
    r#type: String,
    scope: String,
    subject: String,
}

impl CommitMessage {
    fn new() -> Self {
        CommitMessage {
            r#type: String::new(),
            scope: String::new(),
            subject: String::new(),
        }
    }

    fn set_type(&mut self, value: String) {
        self.r#type = value
    }

    fn set_scope(&mut self, value: String) {
        self.scope = value
    }

    fn set_subject(&mut self, value: String) {
        self.subject = value
    }

    fn to_string(&self) -> String {
        if self.scope.trim().is_empty() {
            format!("{}: {}", self.r#type, self.subject)
        } else {
            format!("{}({}): {}", self.r#type, self.scope, self.subject)
        }
    }
}

impl Display for CommitMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Commit Message:").expect("Error: display CommitMessage");
        writeln!(f, "type: {}", self.r#type).expect("Error: display type filed in CommitMessage");
        writeln!(f, "scope: {}", self.scope).expect("Error: display scope filed in CommitMessage");
        writeln!(f, "subject: {}", self.subject)
            .expect("Error: display subject filed in CommitMessage");

        Ok(())
    }
}

pub fn handle_commit() {
    let mut commit_message = CommitMessage::new();

    let types: Vec<&str> = vec![
        "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "cli", "chore",
        "revert",
    ];
    let r#type = Select::new("Select the type of change that you're committing: ", types).prompt();
    match r#type {
        Ok(r#type) => {
            commit_message.set_type(r#type.to_string());
        }
        Err(_) => {
            println!("There was an error, please try again")
        }
    }

    let scope =
        Text::new("What is the scope of this change (e.g. component or file name): ").prompt();
    match scope {
        Ok(scope) => commit_message.set_scope(scope),
        Err(_) => {
            println!("There was an error, please try again")
        }
    }

    let subject = Text::new("Write a short, imperative tense description of the change: ")
        .with_validator(required!("Subject cannot be empty."))
        .prompt();
    match subject {
        Ok(subject) => commit_message.set_subject(subject),
        Err(_) => {
            println!("There was an error, please try again")
        }
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
