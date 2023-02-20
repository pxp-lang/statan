use std::{slice::Iter, fmt::Display};

#[derive(Debug, Clone)]
pub struct Message {
    pub severity: MessageSeverity,
    pub message: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum MessageSeverity {
    Error,
    Warning,
    Note,
}

impl Display for MessageSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageSeverity::Error => write!(f, "Error"),
            MessageSeverity::Warning => write!(f, "Warning"),
            MessageSeverity::Note => write!(f, "Note"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MessageCollector {
    file: String,
    messages: Vec<Message>,
}

impl MessageCollector {
    pub fn new(file: String) -> Self {
        Self {
            file,
            messages: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<Message> {
        self.messages.iter()
    }

    pub fn get_file(&self) -> &str {
        self.file.as_str()
    }

    pub fn error(&mut self, message: impl Into<String>, line: usize) {
        self.messages.push(Message {
            severity: MessageSeverity::Error,
            message: message.into(),
            line,
        });
    }

    pub fn warning(&mut self, message: impl Into<String>, line: usize) {
        self.messages.push(Message {
            severity: MessageSeverity::Warning,
            message: message.into(),
            line,
        });
    }

    pub fn note(&mut self, message: impl Into<String>, line: usize) {
        self.messages.push(Message {
            severity: MessageSeverity::Note,
            message: message.into(),
            line,
        });
    }
}