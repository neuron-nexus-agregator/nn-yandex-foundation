pub mod models;

use reqwest;
use models::request::Request;
use core::config::YANDEX_GPT_URL;
use models::response::Result as YandexResult;
use models::response::Error as YandexError;
use std::future::Future;

/// Supported model types
pub enum ModelType {
    GptLite,
    GptPro,
    Llama8B,
    Llama70B,
}

impl ModelType {
    /// Returns the model URI segment as string
    pub fn as_str(&self) -> &str {
        match self {
            ModelType::GptLite => "yandexgpt-lite",
            ModelType::GptPro => "yandexgpt",
            ModelType::Llama8B => "llama-lite",
            ModelType::Llama70B => "llama",
        }
    }
}

/// Supported versions
pub enum Version {
    Deprecated,
    Latest,
    RC,
}

impl Version {
    /// Returns the version as string
    pub fn as_str(&self) -> &str {
        match self {
            Version::Deprecated => "deprecated",
            Version::Latest => "latest",
            Version::RC => "rc",
        }
    }
}

/// Errors returned by the TextGenerator
#[derive(Debug)]
pub enum GeneratorError {
    /// HTTP request error
    Http(reqwest::Error),
    /// Error returned by Yandex API
    Api(YandexError),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::Http(e) => write!(f, "HTTP error: {}", e),
            GeneratorError::Api(e) => write!(f, "API error: {:?}", e),
            GeneratorError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for GeneratorError {}

/// Main structure for text generation
pub struct TextGenerator {
    api_key: String,
    bucket_id: String,
    client: reqwest::Client,
}

impl TextGenerator {
    /// Creates a new TextGenerator instance
    pub fn new(api_key: String, bucket_id: String) -> Self {
        Self {
            api_key,
            bucket_id,
            client: reqwest::Client::new(),
        }
    }

    /// Changes API key and bucket ID
    pub fn change_credentials(&mut self, api_key: String, bucket_id: String) {
        self.api_key = api_key;
        self.bucket_id = bucket_id;
    }

    /// Sends a text completion request
    /// Returns a Future instead of being async
    pub fn complete(
        &self,
        model: ModelType,
        version: Version,
        mut request: Request,
    ) -> impl Future<Output = Result<YandexResult, GeneratorError>> + '_ {
        let client = &self.client;
        let api_key = &self.api_key;
        let bucket_id = &self.bucket_id;

        async move {
            request.model_uri = format!("gpt://{}/{}/{}", bucket_id, model.as_str(), version.as_str());
            if let Some(opts) = request.completion_options.as_mut() {
                opts.stream = Some(false);
            }

            let resp = client
                .post(YANDEX_GPT_URL)
                .header("Authorization", format!("Api-Key {}", api_key))
                .json(&request)
                .send()
                .await
                .map_err(GeneratorError::Http)?;

            let status = resp.status().clone();

            if !resp.status().is_success() {
                match resp.json::<YandexError>().await {
                    Ok(err) => return Err(GeneratorError::Api(err)),
                    Err(_) => return Err(GeneratorError::Unknown(format!("request failed with status: {status}"))),
                }
            }

            let result = resp.json::<YandexResult>().await.map_err(GeneratorError::Http)?;
            Ok(result)
        }
    }
}
