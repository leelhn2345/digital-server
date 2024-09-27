use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct OpenAISettings {
    pub chat: ChatSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChatSettings {
    pub model: String,
    pub max_tokens: u16,
    pub past_log_count: i64,
}
