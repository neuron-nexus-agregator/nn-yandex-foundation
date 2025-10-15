use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: Role,
    
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_list: Option<ToolCallList>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_result_list: Option<ToolResultList>,
}

pub struct MessageBuilder{
    role: Option<Role>,
    text: Option<String>,
    tool_call_list: Option<ToolCallList>,
    tool_result_list: Option<ToolResultList>
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder { role: None, text: None, tool_call_list: None, tool_result_list: None }
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn with_tool_call_list(mut self, tool_call_list: &ToolCallList) -> Self {
        self.tool_call_list = Some(tool_call_list.clone());
        self
    }
    
    pub fn with_tool_result_list(mut self, tool_result_list: &ToolResultList) -> Self {
        self.tool_result_list = Some(tool_result_list.clone());
        self
    }

    pub fn build(self) -> Message {
        Message { 
            role: self.role.unwrap_or(Role::User), 
            text: self.text.unwrap_or_default(), 
            tool_call_list: self.tool_call_list, 
            tool_result_list: self.tool_result_list 
        }
    }
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


