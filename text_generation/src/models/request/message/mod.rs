use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_list: Option<ToolCallList>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_result_list: Option<ToolResultList>,

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
pub struct ToolCallList {
    pub tool_calls: Vec<ToolCallWrapper>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallWrapper {
    pub function_call: FunctionCall,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Value,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultList {
    pub tool_results: Vec<ToolResultWrapper>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultWrapper {
    pub function_result: FunctionResult,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResult {
    pub name: String,
    pub content: String,
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
