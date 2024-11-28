mod llms;

use std::io::{self, Write, Read};
use clap::{Parser, ValueEnum};
use llms::LlmModel;
use llms::openai::OpenAiModel;
use llms::anthropic::AnthropicModel;

#[derive(Clone, ValueEnum)]
enum Provider {
    OpenAI,
    Anthropic,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mode to run in (chat or api)
    #[arg(value_enum)]
    mode: Mode,

    /// LLM provider to use
    #[arg(value_enum)]
    provider: Provider,

    /// Model name to use
    #[arg(long)]
    model: String,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Chat,
    Api,
}

fn create_model(provider: Provider, model: String) -> Box<dyn LlmModel> {
    match provider {
        Provider::OpenAI => {
            let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
            Box::new(OpenAiModel::new(api_key, model))
        }
        Provider::Anthropic => {
            let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
            Box::new(AnthropicModel::new(api_key, model))
        }
    }
}

async fn chat_mode(model: Box<dyn LlmModel>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting chat with {} from {}. Type 'exit' to quit.", 
             model.model_name(), model.provider());
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        let response = model.query(input).await?;
        println!("\n{}\n", response);
    }
    
    Ok(())
}

async fn api_mode(model: Box<dyn LlmModel>) -> Result<(), Box<dyn std::error::Error>> {
    println!("API mode: Reading from stdin, writing to stdout");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let response = model.query(&input).await?;
    println!("{}", response);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    let model = create_model(cli.provider, cli.model);
    
    match cli.mode {
        Mode::Chat => chat_mode(model).await?,
        Mode::Api => api_mode(model).await?,
    }
    
    Ok(())
}
