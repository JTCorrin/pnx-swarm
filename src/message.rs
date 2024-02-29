use crate::prelude::*;

#[derive(Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Default)]
pub struct MessageBuilder {
    role: Option<String>,
    content: Option<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder::default()
    }

    pub fn role(&mut self, role: impl Into<String>) -> &mut Self {
        self.role.insert(role.into());
        self
    }

    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        self.content.insert(content.into());
        self
    }

    pub fn build(&self) -> Result<Message> {
        let role = self.role.as_ref().ok_or(Error::Generic("Message role is required".to_string()))?;
        let content = self.content.as_ref().ok_or(Error::Generic("Message content is required".to_string()))?;

        Ok(Message {
            role: role.to_string(),
            content: content.to_string(),
        })
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            role: "Role not yet set".to_string(),
            content: "No content".to_string(),
        }
    }
}

impl Message {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Message {
        Message {
            role: role.into(),
            content: content.into(),
        }
    }
}

#[cfg(test)]
#[path = "_tests/message.rs"]
mod tests;