mod llms;

use llms::LlmModel;
use llms::openai::OpenAiModel;
use llms::anthropic::AnthropicModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // OpenAI examples with different models
    let openai_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    
    let models = vec![
        OpenAiModel::new(openai_key.clone(), "gpt-4".to_string()),
        OpenAiModel::new(openai_key.clone(), "gpt-3.5-turbo".to_string()),
    ];

    for model in models {
        println!("Testing {} from {}", model.model_name(), model.provider());
        let response = model.query("Hello, how are you?").await?;
        println!("Response: {}\n", response);
    }
    
    // Anthropic examples with different models
    let anthropic_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
    
    let models = vec![
        AnthropicModel::new(anthropic_key.clone(), "claude-3-opus-20240229".to_string()),
        AnthropicModel::new(anthropic_key.clone(), "claude-3-sonnet-20240229".to_string()),
    ];

    for model in models {
        println!("Testing {} from {}", model.model_name(), model.provider());
        let response = model.query("Hello, how are you?").await?;
        println!("Response: {}\n", response);
    }
    
    Ok(())
}
