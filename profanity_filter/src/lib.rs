pub struct Message {
    pub content: String,
    pub user: String,
}

impl Message {
    pub fn new(content: &str, user: &str) -> Self {
        Message {
            content: content.to_string(),
            user: user.to_string(),
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.contains("stupid") || self.content.trim().is_empty() {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<String, &str> {
    let text = Message::new(message, "kuya");
    let res = text.send_ms();

    if let Some(content) = res {
        Ok(content.to_string())
    } else {
        Err("ERROR: illegal")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
