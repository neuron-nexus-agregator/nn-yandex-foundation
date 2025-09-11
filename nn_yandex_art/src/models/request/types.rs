/// Enum for image mime types
#[derive(Debug)]
pub enum ImageType{
    Jpeg,
    Png,
}

impl ImageType {
    pub fn to_string(&self) -> String {
        let prefix = "image/".to_string();
        match self {
            ImageType::Jpeg => prefix + "jpeg",
            ImageType::Png => prefix + "png",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_type_to_string() {
        assert_eq!(ImageType::Jpeg.to_string(), "image/jpeg");
        assert_eq!(ImageType::Png.to_string(), "image/png");
    }
}