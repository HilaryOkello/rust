struct Message {
    content: String,
    user: String,
}

//new initializes Message
impl Message {
    pub fn new(content: String, user: String) -> Self {
        Message { content, user }
    }

    pub fn send_ms(self) -> Option<String> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg =  Message::new(message.to_string(), "user".to_string());

    match msg.send_ms() {
        Some(_) => Ok("Message sent"),
        None => Err("Message contains profanity"),
    }

}