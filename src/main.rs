mod llms;

use std::io::{self, Write};
use clap::{Parser, ValueEnum};
use llms::LlmModel;
use llms::openai::OpenAiModel;
use llms::anthropic::AnthropicModel;
use axum::{
    routing::{post, get},
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
struct QueryRequest {
    model_name: String,
    prompt: String,
}

#[derive(Serialize)]
struct QueryResponse {
    response: String,
}

#[derive(Serialize)]
struct ModelInfo {
    model_name: String,
    provider: String,
}

async fn api_mode(model: Box<dyn LlmModel>) -> Result<(), Box<dyn std::error::Error>> {
    let model = std::sync::Arc::new(model);
    
    let router = Router::new()
        .route("/query", post(handle_query))
        .route("/models", get(handle_list_models))
        .with_state(model);

    let address = "0.0.0.0:3000".parse::<std::net::SocketAddr>()?;
    println!("Starting API server on http://localhost:3000");
    axum::serve(
        tokio::net::TcpListener::bind(address).await?, 
        router
    ).await?;
    
    Ok(())
}

async fn handle_query(
    State(model): State<std::sync::Arc<Box<dyn LlmModel>>>,
    Json(request): Json<QueryRequest>,
) -> Json<QueryResponse> {
    // TODO: Add model selection logic based on request.model_name
    let response = model.query(&request.prompt).await
        .unwrap_or_else(|e| format!("Error: {}", e));
    
    Json(QueryResponse { response })
}

async fn handle_list_models(
    State(model): State<std::sync::Arc<Box<dyn LlmModel>>>,
) -> Json<Vec<ModelInfo>> {
    // For now, just return the single configured model
    Json(vec![ModelInfo {
        model_name: model.model_name().to_string(),
        provider: model.provider().to_string(),
    }])
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
