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
