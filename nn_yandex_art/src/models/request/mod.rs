pub mod message;
pub mod aspect_ratio;
pub mod generation_options;
pub mod types;

use message::Message;
use generation_options::GenerationOptions;

use serde::Serialize;
use crate::error::BuildError;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request{
    pub(crate) model_uri: String,
    messages: Vec<Message>,
    generation_options: GenerationOptions,
}


pub struct RequestBuilder {
    messages: Vec<Message>,
    generation_options: Option<GenerationOptions>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            generation_options: None,
        }
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn generation_options(mut self, generation_options: GenerationOptions) -> Self {
        self.generation_options = Some(generation_options);
        self
    }

    pub fn build(self) -> Result<Request, BuildError> {

        if self.messages.is_empty() {
            return Err(BuildError::new("No messages added"));
        }

        if self.generation_options.is_none() {
            return Err(BuildError::new("No generation options added"));
        }

        Ok(Request {
            model_uri: "".to_string(),
            messages: self.messages,
            generation_options: self.generation_options.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request::message::MessageBuilder;
    use crate::models::request::generation_options::GenerationOptionsBuilder;
    use crate::models::request::aspect_ratio::AspectRatioBuilder;
    use crate::models::request::types::ImageType;

    #[test]
    fn test_request_builder_success() {
        let message = MessageBuilder::new()
            .text("Hello")
            .weight(1)
            .build()
            .unwrap();

        let aspect_ratio = AspectRatioBuilder::new()
            .width_ratio(1)
            .height_ratio(1)
            .build();

        let options = GenerationOptionsBuilder::new()
            .mime_type(ImageType::Png)
            .aspect_ratio(aspect_ratio)
            .build()
            .unwrap();

        let request = RequestBuilder::new()
            .message(message)
            .generation_options(options)
            .build()
            .unwrap();

        assert_eq!(request.messages.len(), 1);
    }

    #[test]
    fn test_request_builder_missing_messages() {
        let aspect_ratio = AspectRatioBuilder::new()
            .width_ratio(1)
            .height_ratio(1)
            .build();

        let options = GenerationOptionsBuilder::new()
            .mime_type(ImageType::Jpeg)
            .aspect_ratio(aspect_ratio)
            .build()
            .unwrap();

        let result = RequestBuilder::new()
            .generation_options(options)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No messages added");
    }

    #[test]
    fn test_request_builder_missing_generation_options() {
        let message = MessageBuilder::new()
            .text("Hello")
            .weight(1)
            .build()
            .unwrap();

        let result = RequestBuilder::new()
            .message(message)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No generation options added");
    }
}
