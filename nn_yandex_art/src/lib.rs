pub mod models;
pub mod error;

use models::request::Request;
use models::response::Response;
use std::future::Future;
use crate::error::ArtError;

const YANDEX_ART_URL: &str = "https://llm.api.cloud.yandex.net/foundationModels/v1/imageGenerationAsync";
const YANDEX_GET_OPERATION: &str = "https://operation.api.cloud.yandex.net/operations";



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
    /// `Result<Response, ArtError>` - The response with `id` and `done`-flag or error
    pub fn generate_image(
        &self,
        mut request: Request,
    ) -> impl Future<Output = Result<Response, ArtError>> + '_ {
        request.model_uri = format!("art://{}/yandex-art/latest", self.bucket_id);
        let client = &self.client;
        let api_key = &self.api_key;

        async move {
            let resp = client
                .post(YANDEX_ART_URL)
                .header("Authorization", format!("Api-Key {}", api_key))
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
    }

    /// Checks the status of an image generation operation
    ///
    /// # Arguments
    ///
    /// * `request_id` - ID of the request from `response` returned by `generate_image`
    ///
    /// # Returns
    ///
    /// `Result<Response, ArtError>` - Current operation status or error
    pub fn check_operation(
        &self,
        request_id: &str,
    ) -> impl Future<Output = Result<Response, ArtError>> + '_ {
        let client = &self.client;
        let api_key = &self.api_key;
        let request_id = request_id.to_string();

        async move {
            let resp = client
                .get(format!("{YANDEX_GET_OPERATION}/{request_id}"))
                .header("Authorization", format!("Api-Key {}", api_key))
                .send()
                .await
                .map_err(ArtError::Http)?;

            let text = resp.text().await.map_err(ArtError::Http)?;

            let result: Response = serde_json::from_str(&text).map_err(|e| {
                ArtError::Api(format!("Failed to parse JSON: {e}. Response text: {text}"))
            })?;

            if let Some(err) = &result.error {
                return Err(ArtError::Api(err.message.clone()));
            }

            Ok(result)
        }
    }
}
