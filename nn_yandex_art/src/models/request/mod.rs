use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message{
    pub text: String,
    pub weight: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AspectRatio{
    pub width_ratio: i64,
    pub height_ratio: i64,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerationOptions{
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    pub aspect_ratio: AspectRatio,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request{
    pub(crate) model_uri: String,
    pub messages: Vec<Message>,
    pub generation_options: GenerationOptions,
}

impl Request{
    pub fn new(messages: Vec<Message>, options: GenerationOptions) ->Self{
        Request { model_uri: "".to_string(), messages: messages, generation_options: options }
    }
}
