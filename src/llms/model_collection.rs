use std::collections::HashMap;
use super::LlmModel;
use super::openai::OpenAiModel;
use super::anthropic::AnthropicModel;

pub struct ModelCollection {
    models: HashMap<String, Box<dyn LlmModel>>,
}

impl ModelCollection {
    pub fn new() -> Self {
        let mut models : HashMap<String, Box<dyn LlmModel>> = HashMap::new();
        
        // OpenAI Models
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            models.insert(
                "gpt-4o".to_string(),
                Box::new(OpenAiModel::new(api_key.clone(), "gpt-4o".to_string()))
            );
            models.insert(
                "gpt-3.5-turbo".to_string(),
                Box::new(OpenAiModel::new(api_key, "gpt-3.5-turbo".to_string()))
            );
        }
        
        // Anthropic Models
        if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            models.insert(
                "claude-3-sonnet".to_string(),
                Box::new(AnthropicModel::new(api_key.clone(), "claude-3-5-sonnet-latest".to_string()))
            );
        }
        
        Self { models }
    }
    
    pub fn get_model(&self, model_name: &str) -> Option<&Box<dyn LlmModel>> {
        self.models.get(model_name)
    }
    
    pub fn list_models(&self) -> Vec<(&String, &str, &str)> {
        self.models
            .iter()
            .map(|(name, model)| (name, model.provider(), model.model_name()))
            .collect()
    }
} 