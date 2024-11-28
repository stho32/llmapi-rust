mod llms;

use llms::LlmModel;
use llms::openai::OpenAiModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let model = OpenAiModel::new(api_key);
    
    let response = model.query("Hello, how are you?").await?;
    println!("Response: {}", response);
    
    Ok(())
}
