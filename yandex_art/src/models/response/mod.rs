use serde::Deserialize;
use serde_json::Value;
use time::OffsetDateTime;


#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWrapper{
    pub image: String,
    pub model_version: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorWrapper{
    pub code: i64,
    pub message: String,
    pub details: Option<Vec<Value>>,
}   


#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response{
    pub id: String,
    pub description: Option<String>,

    #[serde(default, with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    pub created_by: Option<String>,

    #[serde(default, with = "time::serde::rfc3339::option")]
    pub modified_at: Option<OffsetDateTime>,

    pub done: bool,
    pub metadata: Option<Value>,
    pub error: Option<ErrorWrapper>,
    pub response: Option<ResponseWrapper>,
}