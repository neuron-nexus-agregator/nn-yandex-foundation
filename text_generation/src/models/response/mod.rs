use crate::models::message::Message;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

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

#[serde_as]
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Usage{
    #[serde_as(as = "DisplayFromStr")]
    pub input_text_tokens: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub completion_tokens: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub total_tokens: i64,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[serde_as]
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionTokensDetails{
    #[serde_as(as = "DisplayFromStr")]
    pub reasoning_tokens: i64,
}