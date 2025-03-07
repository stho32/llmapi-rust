use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::llms::LlmModel;
use tokio::time;

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
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
}

#[async_trait]
impl LlmModel for AnthropicModel {
    fn model_name(&self) -> &str {
        &self.model
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

        let mut response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Retry up to 10 times if we get a 529 status code
        let mut attempts = 1;
        const MAX_ATTEMPTS: u8 = 10;

        while response.status() == 529 && attempts < MAX_ATTEMPTS {
            // Wait for 2 seconds before retrying
            time::sleep(time::Duration::from_secs(2)).await;
            
            // Log retry attempt
            eprintln!("Anthropic API returned 529, retrying (attempt {}/{})...", attempts, MAX_ATTEMPTS-1);
            
            // Retry the request
            response = self.client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await?;
                
            attempts += 1;
        }

        // If we still have a 529 after all attempts, make the error more descriptive
        if response.status() == 529 {
            return Err(format!("Anthropic API returned 529 status code after {} attempts. Service is likely overloaded.", MAX_ATTEMPTS).into());
        }

        // Check for other errors
        // Note: error_for_status() consumes response and returns it if status is success
        response = response.error_for_status()?;

        let response_data: AnthropicResponse = response.json().await?;
        
        Ok(response_data
            .content
            .first()
            .map(|content| content.text.clone())
            .unwrap_or_else(|| String::from("No response generated.")))
    }
} 