use serde::Serialize;
use crate::error::BuildError;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message{
    text: String,
    weight: i64,
}

pub struct MessageBuilder {
    text: String,
    weight: i64,
}

impl MessageBuilder {
    pub fn new() -> Self{
        MessageBuilder{
            text: "".to_string(),
            weight: 1
        }
    }
    
    pub fn text(mut self, text: &str) -> Self{
        self.text = text.to_string();
        self
    }
    
    pub fn weight(mut self, weight: i64) -> Self{
        self.weight = weight;
        self
    }
    
    pub fn build(self) -> Result<Message, BuildError> {
        if self.text.is_empty() {
            return Err(BuildError::new("Text is required"))
        }
        Ok(Message{
            text: self.text,
            weight: self.weight
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_builder_success() {
        let message = MessageBuilder::new()
            .text("Hello")
            .weight(2)
            .build()
            .unwrap();

        assert_eq!(message.text, "Hello");
        assert_eq!(message.weight, 2);
    }

    #[test]
    fn test_message_builder_missing_text() {
        let result = MessageBuilder::new().weight(1).build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Text is required");
    }
}