use crate::models::message::Message;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Result{
    pub result: ResultWrapper,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultWrapper{
    pub alternatives: Vec<Alternative>,
    pub usage: Usage,
    pub model_version: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Error{
    pub code: String,
    pub message: String,    
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Alternative{
    pub message: Message,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Usage{
    pub input_text_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionTokensDetails{
    pub reasoning_tokens: i64,
}