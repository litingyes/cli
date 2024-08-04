use std::fmt::{Display, Formatter};

pub struct CommitMessage {
    r#type: String,
    scope: String,
    subject: String,
    has_break_change: bool,
}

impl CommitMessage {
    pub fn new() -> Self {
        CommitMessage {
            r#type: String::new(),
            scope: String::new(),
            subject: String::new(),
            has_break_change: false,
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

    pub fn set_has_break_change(&mut self, value: bool) {
        self.has_break_change = value
    }

    pub fn to_string(&self) -> String {
        let mut commit_msg = self.r#type.to_string();

        if !self.scope.trim().is_empty() {
            commit_msg.push_str(format!("({})", self.scope.as_str()).as_str())
        }
        if self.has_break_change == true {
            commit_msg.push_str("!")
        }
        commit_msg.push_str(": ");
        commit_msg.push_str(self.subject.as_str());

        commit_msg
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
