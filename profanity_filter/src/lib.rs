pub struct Message {
    pub content: String,
    pub user: String,
}

impl Message {
    pub fn new(content: &str) -> Self {
        Message {
            content: content.to_string(),
            user: String::from("anonymous"),
        }
    }

    pub fn send_ms(&self) -> Option<String> {
        if self.content.contains("stupid") || self.content.trim().is_empty() {
            None
        } else {
            Some(self.content.clone()) // Return owned String
        }
    }
}

pub fn check_ms(message: &str) -> Result<String, &'static str> {
    if let Some(content) = Message::new(message).send_ms() {
        Ok(content)
    } else {
        Err("ERROR: illegal")
    }
}
