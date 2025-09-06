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

pub struct TextGenerator{
    api_key: String,
    bucket_id: String,
}

impl TextGenerator{
    pub fn new(api_key: String, bucket_id: String) -> Self{
        Self{
            api_key,
            bucket_id,
        }
    }

    pub fn change_credentials(&mut self, api_key: String, bucket_id: String){
        self.api_key = api_key;
        self.bucket_id = bucket_id;
    }

    pub async fn complete(&self, model: ModelType, version: Version, mut request: Request) -> Result<YandexResult, Box<dyn std::error::Error>>{
        request.model_uri = format!("gpt://{}/{}/{}", self.bucket_id, model.as_str(), version.as_str());
        if let Some(opts) = request.completion_options.as_mut() {
            opts.stream = Some(false);
        }
        let client = reqwest::Client::new();
        let resp = client
            .post(YANDEX_GPT_URL)
            .header("Authorization", format!("Api-Key {}", self.api_key))
            .json(&request)
        .send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            match resp.json::<YandexError>().await {
                Ok(err) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err))));
                }
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("request failed with status: {}", status))));
                }
            }
        }

        let result = resp.json::<YandexResult>().await?;
        Ok(result)

    }
    
}