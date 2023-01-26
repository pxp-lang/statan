#[derive(Debug)]
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

    pub fn add(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }
}
