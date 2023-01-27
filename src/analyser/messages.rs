use std::slice::Iter;

#[derive(Debug, Default, Clone)]
pub struct MessageCollector {
    file: String,
    messages: Vec<String>,
}

impl MessageCollector {
    pub fn new(file: String) -> Self {
        Self {
            file,
            messages: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<String> {
        self.messages.iter()
    }

    pub fn get_file(&self) -> &str {
        self.file.as_str()
    }

    pub fn add(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }
}