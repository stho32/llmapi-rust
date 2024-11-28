mod llms;

use std::io::{self, Write, Read};
use clap::{Parser, ValueEnum};
use llms::LlmModel;
use llms::openai::OpenAiModel;
use llms::anthropic::AnthropicModel;
use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

async fn api_mode(model: Box<dyn LlmModel>) -> Result<(), Box<dyn std::error::Error>> {
    let model = std::sync::Arc::new(model);
    
    let app = Router::new()
        .route("/chat", post(handle_chat))
        .with_state(model);

    println!("Starting API server on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

async fn handle_chat(
    State(model): State<std::sync::Arc<Box<dyn LlmModel>>>,
    Json(request): Json<ChatRequest>,
) -> Json<ChatResponse> {
    let response = model.query(&request.message).await
        .unwrap_or_else(|e| format!("Error: {}", e));
    
    Json(ChatResponse { response })
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
