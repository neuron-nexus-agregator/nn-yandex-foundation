pub mod models;

use models::request::Request;
use models::response::Response;
use core::config::{YANDEX_ART_URL, YANDEX_GET_OPERATION};

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

impl std::error::Error for ArtError {}

/// Main structure for generating images with Yandex Art API
pub struct Art {
    api_key: String,
    bucket_id: String,
    client: reqwest::Client,
}

impl Art {
    /// Creates a new Art instance
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Yandex API key
    /// * `bucket_id` - The bucket ID for the model
    pub fn new(api_key: String, bucket_id: String) -> Self {
        Self {
            api_key,
            bucket_id,
            client: reqwest::Client::new(),
        }
    }

    /// Changes API key and bucket ID
    ///
    /// # Arguments
    ///
    /// * `api_key` - New API key
    /// * `bucket_id` - New bucket ID
    pub fn change_credentials(&mut self, api_key: String, bucket_id: String) {
        self.api_key = api_key;
        self.bucket_id = bucket_id;
    }

    /// Sends a request to generate an image asynchronously
    ///
    /// # Arguments
    ///
    /// * `request` - Request data for image generation
    ///
    /// # Returns
    ///
    /// `Result<Response, ArtError>` - The response or error
    pub async fn generate_image_async(&self, mut request: Request) -> Result<Response, ArtError> {
        request.model_uri = format!("art://{}/yandex-art/latest", self.bucket_id);

        let resp = self
            .client
            .post(YANDEX_ART_URL)
            .header("Authorization", format!("Api-Key {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(ArtError::Http)?;

        let result: Response = resp.json().await.map_err(ArtError::Http)?;

        if let Some(err) = &result.error {
            return Err(ArtError::Api(err.message.clone()));
        }

        Ok(result)
    }

    /// Checks the status of an image generation operation
    ///
    /// # Arguments
    ///
    /// * `request_id` - ID of the request returned by `generate_image_async`
    ///
    /// # Returns
    ///
    /// `Result<Response, ArtError>` - Current operation status or error
    pub async fn check_operation(&self, request_id: &str) -> Result<Response, ArtError> {
        let resp = self
            .client
            .get(format!("{YANDEX_GET_OPERATION}/{request_id}"))
            .header("Authorization", format!("Api-Key {}", self.api_key))
            .send()
            .await
            .map_err(ArtError::Http)?;

        let text = resp.text().await.map_err(ArtError::Http)?;

        let result: Response = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                return Err(ArtError::Api(format!(
                    "Failed to parse JSON: {e}. Response text: {text}"
                )))
            }
        };

        if let Some(err) = &result.error {
            return Err(ArtError::Api(err.message.clone()));
        }

        Ok(result)
    }
}
