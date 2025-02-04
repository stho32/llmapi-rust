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
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseChoice {
    message: OpenAIResponseMessage,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseMessage {
    content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<OpenAIResponseChoice>,
}

#[derive(Deserialize, Debug)]
struct OpenAIError {
    error: OpenAIErrorDetails,
}

#[derive(Deserialize, Debug)]
struct OpenAIErrorDetails {
    message: String,
    #[serde(default)]
    r#type: String,
    #[serde(default)]
    code: String,
}

impl OpenAiModel {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }

    fn get_reasoning_effort(&self) -> Option<String> {
        if self.model == "o3-mini" {
            Some("medium".to_string())
        } else {
            None
        }
    }
}

#[async_trait]
impl LlmModel for OpenAiModel {
    fn model_name(&self) -> &str {
        &self.model
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
            reasoning_effort: self.get_reasoning_effort(),
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let response_text = response.text().await?;
        
        // Try to parse as successful response
        if let Ok(response_data) = serde_json::from_str::<OpenAIResponse>(&response_text) {
            return Ok(response_data
                .choices
                .first()
                .map(|choice| choice.message.content.clone())
                .unwrap_or_else(|| String::from("No response generated.")));
        }
        
        // Try to parse as error response
        if let Ok(error_data) = serde_json::from_str::<OpenAIError>(&response_text) {
            return Err(format!(
                "OpenAI API Error: {} (Type: {}, Code: {})", 
                error_data.error.message,
                error_data.error.r#type,
                error_data.error.code
            ).into());
        }
        
        // If neither parsing worked, return the raw response
        Err(format!("Unexpected API response: {}", response_text).into())
    }
} 