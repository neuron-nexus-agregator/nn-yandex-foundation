#[derive(Debug)]
pub struct BuildError {
    message: String,
}

impl BuildError {
    pub fn new(message: &str) -> Self {
        BuildError { message: message.to_string() }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BuildError {}

/// Errors returned by the Art library
#[derive(Debug)]
pub enum ArtError {
    /// HTTP request error
    Http(reqwest::Error),
    /// Error returned by Yandex API
    Api(String),
    /// Operation is not yet finished
    NotReady,
    /// Response field is missing in the result
    MissingResponse,
}

impl std::fmt::Display for ArtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtError::Http(e) => write!(f, "HTTP error: {}", e),
            ArtError::Api(msg) => write!(f, "API error: {}", msg),
            ArtError::NotReady => write!(f, "Operation not finished"),
            ArtError::MissingResponse => write!(f, "Response missing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_error_display() {
        let err = BuildError::new("Something went wrong");
        assert_eq!(format!("{}", err), "Something went wrong");
    }

    #[test]
    fn test_build_error_debug() {
        let err = BuildError::new("Debug test");
        assert_eq!(format!("{:?}", err), "BuildError { message: \"Debug test\" }");
    }
}
