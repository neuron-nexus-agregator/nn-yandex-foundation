use serde::Serialize;
use serde_json::Value;
use crate::models::message::Message;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request{
    pub model_uri: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_options: Option<CompletionOptions>,

    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<FunctionWrapper>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_object: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<JsonSchema>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
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

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionWrapper {
    pub function: Function,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub strict: bool,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    pub schema: Value,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolChoice {
    pub mode: String,
    pub function_name: String,
}