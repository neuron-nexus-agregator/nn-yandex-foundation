use serde::Serialize;
use crate::models::request::aspect_ratio::AspectRatio;
use crate::error::BuildError;
use crate::models::request::types::ImageType;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerationOptions{
    mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
    aspect_ratio: AspectRatio,
}

/// Builder for GenerationOptions
/// `mime_type` & `aspect_ratio` are required
#[derive(Debug)]
pub struct GenerationOptionsBuilder {
    mime_type: Option<String>,
    seed: Option<i64>,
    aspect_ratio: Option<AspectRatio>,
}


/// Builder for GenerationOptions
/// `mime_type` & `aspect_ratio` are required
impl GenerationOptionsBuilder {
    pub fn new() -> Self {
        GenerationOptionsBuilder {
            mime_type: None,
            seed: None,
            aspect_ratio: None,
        }
    }
    
    pub fn mime_type(mut self, mime_type: ImageType) -> Self {
        self.mime_type = Some(mime_type.to_string());
        self
    }
    
    pub fn seed(mut self, seed: i64) -> Self {
        self.seed = Some(seed);
        self
    }
    
    pub fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }
    
    pub fn build(self) -> Result<GenerationOptions, BuildError> {
        if self.mime_type.is_none() {
            return Err(BuildError::new("Mime type is required"));
        }
        
        if self.aspect_ratio.is_none() {
            return Err(BuildError::new("Aspect ratio is required"));
        }
        
        Ok(GenerationOptions {
            mime_type: self.mime_type.unwrap(),
            seed: self.seed,
            aspect_ratio: self.aspect_ratio.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request::aspect_ratio::{AspectRatioBuilder};

    #[test]
    fn test_generation_options_builder_success() {
        let ar = AspectRatioBuilder::new().build();

        let options = GenerationOptionsBuilder::new()
            .mime_type(ImageType::Png)
            .seed(42)
            .aspect_ratio(ar.clone())
            .build()
            .unwrap();

        assert_eq!(options.mime_type, "image/png");
        assert_eq!(options.seed, Some(42));
    }

    #[test]
    fn test_generation_options_builder_missing_mime_type() {
        let ar = AspectRatioBuilder::new().build();
        let result = GenerationOptionsBuilder::new()
            .aspect_ratio(ar)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Mime type is required");
    }

    #[test]
    fn test_generation_options_builder_missing_aspect_ratio() {
        let result = GenerationOptionsBuilder::new()
            .mime_type(ImageType::Jpeg)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Aspect ratio is required");
    }
}