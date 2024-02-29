#[cfg(test)]
mod tests {
    use crate::message::{MessageBuilder, Message};

    #[test]
    fn test_message() {
        let message = Message::new("Admin", "Hello, world!");
        assert_eq!(message.role, "Admin");
        assert_eq!(message.content, "Hello, world!");
    }

    #[test]
    fn test_message_builder() {
        let message = MessageBuilder::new()
            .role("Admin")
            .content("Hello, world!")
            .build()
            .unwrap();
        assert_eq!(message.role, "Admin");
        assert_eq!(message.content, "Hello, world!");
    }

    #[test]
    fn test_message_builder_missing_role() {
        let message = MessageBuilder::new()
            .content("Hello, world!")
            .build();
        assert!(message.is_err());
    }

    #[test]
    fn test_message_builder_missing_content() {
        let message = MessageBuilder::new()
            .role("Admin")
            .build();
        assert!(message.is_err());
    }
}