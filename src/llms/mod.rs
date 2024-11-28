pub mod openai;
pub mod anthropic;

use async_trait::async_trait;

#[async_trait]
pub trait LlmModel: Send + Sync {
    fn model_name(&self) -> &str;
    fn provider(&self) -> &str;
    async fn query(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
} 