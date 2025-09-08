pub mod models;

use reqwest;

use models::request::Request;
use core::config::YANDEX_GPT_URL;
use models::response::Result as YandexResult;
use models::response::Error as YandexError;

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


#[derive(Debug)]
pub enum GeneratorError {
    Http(reqwest::Error),
    Api(YandexError),
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

pub struct TextGenerator{
    api_key: String,
    bucket_id: String,
    client: reqwest::Client,
}

impl TextGenerator{
    pub fn new(api_key: String, bucket_id: String) -> Self{
        let client = reqwest::Client::new();
        Self{
            api_key,
            bucket_id,
            client
        }
    }

    pub fn change_credentials(&mut self, api_key: String, bucket_id: String){
        self.api_key = api_key;
        self.bucket_id = bucket_id;
    }

    pub async fn complete(
    &self,
    model: ModelType,
    version: Version,
    mut request: Request,
) -> Result<YandexResult, GeneratorError> {
    request.model_uri = format!("gpt://{}/{}/{}", self.bucket_id, model.as_str(), version.as_str());

    if let Some(opts) = request.completion_options.as_mut() {
        opts.stream = Some(false);
    }

    let resp = self.client
        .post(YANDEX_GPT_URL)
        .header("Authorization", format!("Api-Key {}", self.api_key))
        .json(&request)
        .send()
        .await
        .map_err(GeneratorError::Http)?;

    let status = resp.status().clone();

    if !resp.status().is_success() {
        // Пытаемся разобрать ошибку API
        match resp.json::<YandexError>().await {
            Ok(err) => return Err(GeneratorError::Api(err)),
            Err(_) => return Err(GeneratorError::Unknown(format!("request failed with status: {status}"))),
        }
    }

    let result = resp.json::<YandexResult>().await.map_err(GeneratorError::Http)?;
    Ok(result)
}
}