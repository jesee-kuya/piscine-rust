pub struct Message<'a> {
    pub content: &'a str,
    pub user: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(content: &'a str) -> Self {
        Message {
            content,
            user: "anonymous",
        }
    }

    pub fn send_ms(&self) -> Option<&'a str> {
        if self.content.contains("stupid") || self.content.trim().is_empty() {
            None
        } else {
            Some(self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message);
    msg.send_ms().ok_or("ERROR: illegal")
}
