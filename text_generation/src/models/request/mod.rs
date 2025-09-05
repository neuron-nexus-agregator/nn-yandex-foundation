pub mod message;

use serde::Serialize;
use message::Message;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request{
    pub model_uri: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_options: Option<CompletionOptions>,


    pub messages: Vec<Message>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_options: Option<ReasoningOptions>,

}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReasoningOptions {
    pub mode: String,
}