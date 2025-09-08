pub mod response;
pub mod request;
pub mod message;

pub enum ModelType{
    GptLite,
    GptPro,
    Llama8B,
    Llama70B,
}

impl ModelType {
    pub fn as_str(&self) -> &str {
        match self {
            ModelType::GptLite => "yandexgpt-lite",
            ModelType::GptPro => "yandexgpt",
            ModelType::Llama8B => "llama-lite",
            ModelType::Llama70B => "llama",
        }
    }
}

pub enum Version{
    Deprecated,
    Latest,
    RC
}

impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Version::Deprecated => "deprecated",
            Version::Latest => "latest",
            Version::RC => "rc",
        }
    }
}