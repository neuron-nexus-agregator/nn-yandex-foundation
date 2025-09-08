use serde::Serialize;
use serde_json::Value;
use crate::models::message::Message;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request{
    pub(crate) model_uri: String,

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

impl Request{
    pub fn new(messages: Vec<Message>) -> Self{
        Request { model_uri: "".to_string(), completion_options: None, messages: messages, tools: None, json_object: None, json_schema: None, parallel_tool_calls: None, tool_choice: None }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_options: Option<ReasoningOptions>,


}

impl CompletionOptions {
    pub fn new() -> Self{
        CompletionOptions {
            stream: Some(false),
            temperature: None,
            max_tokens: None,
            reasoning_options: None
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum ReasoningMode{
    #[serde(rename = "REASONING_MODE_UNSPECIFIED")]
    RasoningModeUnspecified,

    #[serde(rename = "DISABLED")]
    Disabled,

    #[serde(rename = "ENABLED_HIDDEN")]
    EnabledHidden
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReasoningOptions {
    pub mode: ReasoningMode,
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
pub enum ToolChoiceMode{
    #[serde(rename = "TOOL_CHOICE_MODE_UNSPECIFIED")]
    ToolChoiceModeUnspecified,

    #[serde(rename = "AUTO")]
    Auto,

    #[serde(rename = "NONE")]
    None,

    #[serde(rename = "REQUIRED")]
    Required,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolChoice {
    pub mode: ToolChoiceMode,
    pub function_name: String,
}