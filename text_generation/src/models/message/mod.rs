use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: Role,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_list: Option<ToolCallList>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_result_list: Option<ToolResultList>,

   
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Role{
    #[serde(rename = "system")]
    System,

    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallList {
    pub tool_calls: Vec<ToolCallWrapper>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallWrapper {
    pub function_call: FunctionCall,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Value,
}

#[derive(Serialize, Deserialize,  Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultList {
    pub tool_results: Vec<ToolResultWrapper>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultWrapper {
    pub function_result: FunctionResult,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResult {
    pub name: String,
    pub content: String,
}


