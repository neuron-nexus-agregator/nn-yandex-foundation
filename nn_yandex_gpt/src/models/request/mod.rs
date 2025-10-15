use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::message::Message;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request{
    pub(crate) model_uri: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) completion_options: Option<CompletionOptions>,

    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<FunctionWrapper>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    json_object: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    json_schema: Option<JsonSchema>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_tool_calls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
}

#[derive(Clone, Debug)]
pub struct RequestBuilder{
    completion_options: Option<CompletionOptions>,
    messages: Vec<Message>,
    tools: Option<Vec<FunctionWrapper>>,
    json_object: Option<bool>,
    json_schema: Option<JsonSchema>,
    parallel_tool_calls: Option<bool>,
    tool_choice: Option<ToolChoice>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder {
            completion_options: None,
            messages: Vec::new(),
            tools: None,
            json_object: None,
            json_schema: None,
            parallel_tool_calls: None,
            tool_choice: None
        }
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }
    pub fn with_completion_options(mut self, completion_options: CompletionOptions) -> Self {
        self.completion_options = Some(completion_options);
        self
    }
    pub fn with_tools(mut self, tools: Vec<FunctionWrapper>) -> Self {
        self.tools = Some(tools);
        self
    }
    pub fn with_json_object(mut self, json_object: bool) -> Self {
        self.json_object = Some(json_object);
        self
    }
    pub fn with_json_schema(mut self, json_schema: JsonSchema) -> Self {
        self.json_schema = Some(json_schema);
        self
    }
    pub fn with_parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }
    pub fn with_tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }
    pub fn build(self) -> Request {
        Request {
            model_uri: "".to_string(),
            completion_options: self.completion_options,
            messages: self.messages,
            tools: self.tools,
            json_object: self.json_object,
            json_schema: self.json_schema,
            parallel_tool_calls: self.parallel_tool_calls,
            tool_choice: self.tool_choice
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions{
    stream: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_options: Option<ReasoningOptions>,


}

impl CompletionOptions {
    pub fn new() -> Self{
        CompletionOptions {
            stream: false,
            temperature: None,
            max_tokens: None,
            reasoning_options: None
        }
    }
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn with_max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn with_reasoning_options(mut self, reasoning_options: ReasoningOptions) -> Self {
        self.reasoning_options = Some(reasoning_options);
        self
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum ReasoningMode{
    #[serde(rename = "REASONING_MODE_UNSPECIFIED")]
    ReasoningModeUnspecified,

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