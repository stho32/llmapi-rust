use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::llms::LlmModel;

pub struct AnthropicModel {
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
struct AnthropicRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct Content {
    text: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<Content>,
}

impl AnthropicModel {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: String::from("claude-3-sonnet-20240229"),
        }
    }
}

#[async_trait]
impl LlmModel for AnthropicModel {
    fn model_name(&self) -> &str {
        "Claude 3 Sonnet"
    }

    fn provider(&self) -> &str {
        "Anthropic"
    }

    async fn query(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: String::from("user"),
                content: prompt.to_string(),
            }],
            max_tokens: 1024,
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Handle potential 529 error with one retry
        if response.status() == 529 {
            let response = self.client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await?;
            response.error_for_status()?;
        }

        let response_data: AnthropicResponse = response.json().await?;
        
        Ok(response_data
            .content
            .first()
            .map(|content| content.text.clone())
            .unwrap_or_else(|| String::from("No response generated.")))
    }
} 