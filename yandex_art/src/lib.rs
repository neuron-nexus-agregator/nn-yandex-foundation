pub mod models;
use models::request::Request;
use models:: response::Response;
use core::config::YANDEX_ART_URL;
use core::config::YANDEX_GET_OPERATION;

pub struct Art{
    api_key: String,
    bucket_id: String,
}

impl Art{
    pub fn new(api_key: String, bucket_id: String) -> Self{
        Art { api_key, bucket_id }
    }

    pub fn change_credentials(&mut self, api_key: String, bucket_id: String){
        self.api_key = api_key;
        self.bucket_id = bucket_id;
    }
}

impl Art{
    pub async fn generate_image_async(&self, mut request: Request) -> Result<Response, reqwest::Error>{
        request.model_uri = format!("gpt://{}/yandex-art/latest", self.bucket_id);
        let client = reqwest::Client::new();
        let resp = client
            .post(YANDEX_ART_URL)
            .header("Authorization", format!("Api-Key {}", self.api_key))
            .json(&request)
        .send().await?;
        let result = resp.json::<Response>().await?;
        Ok(result)
    }

    pub async fn check_operation(&self, request_id: String) -> Result<Response, reqwest::Error>{
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{YANDEX_GET_OPERATION}/{request_id}").to_string())
            .header("Authorization", format!("Api-Key {}", self.api_key))
        .send().await?;
        let result = resp.json::<Response>().await?;
        Ok(result)
    }
}