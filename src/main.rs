mod llms;

use llms::LlmModel;
use llms::openai::OpenAiModel;
use llms::anthropic::AnthropicModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // OpenAI example
    let openai_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let openai_model = OpenAiModel::new(openai_key);
    
    println!("Testing {} from {}", openai_model.model_name(), openai_model.provider());
    let response = openai_model.query("Hello, how are you?").await?;
    println!("Response: {}\n", response);
    
    // Anthropic example
    let anthropic_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
    let anthropic_model = AnthropicModel::new(anthropic_key);
    
    println!("Testing {} from {}", anthropic_model.model_name(), anthropic_model.provider());
    let response = anthropic_model.query("Hello, how are you?").await?;
    println!("Response: {}", response);
    
    Ok(())
}
