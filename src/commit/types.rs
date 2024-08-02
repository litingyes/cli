use std::fmt::{Display, Formatter};

pub struct CommitMessage {
    r#type: String,
    scope: String,
    subject: String,
}

impl CommitMessage {
    pub fn new() -> Self {
        CommitMessage {
            r#type: String::new(),
            scope: String::new(),
            subject: String::new(),
        }
    }

    pub fn set_type(&mut self, value: String) {
        self.r#type = value
    }

    pub fn set_scope(&mut self, value: String) {
        self.scope = value
    }

    pub fn set_subject(&mut self, value: String) {
        self.subject = value
    }

    pub fn to_string(&self) -> String {
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
