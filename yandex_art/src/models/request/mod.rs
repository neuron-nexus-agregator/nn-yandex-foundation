use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message{
    text: String,
    weight: String,
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
