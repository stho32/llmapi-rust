use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::llms::LlmModel;

pub struct OpenAiModel {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct OpenAIResponseChoice {
    message: OpenAIResponseMessage,
}

#[derive(Deserialize)]
struct OpenAIResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIResponseChoice>,
}

impl OpenAiModel {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: String::from("gpt-4"),
        }
    }
}

#[async_trait]
impl LlmModel for OpenAiModel {
    fn model_name(&self) -> &str {
        "GPT-4"
    }

    fn provider(&self) -> &str {
        "OpenAI"
    }

    async fn query(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: String::from("user"),
                content: prompt.to_string(),
            }],
            max_tokens: 1024,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let response_data: OpenAIResponse = response.json().await?;
        
        Ok(response_data
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| String::from("No response generated.")))
    }
} 