use axum::{
    routing::{post, get},
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::llms::model_collection::ModelCollection;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub model_name: String,
    pub prompt: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub response: String,
}

#[derive(Serialize)]
pub struct ModelInfo {
    pub model_name: String,
    pub provider: String,
}

pub async fn run(models: ModelCollection) -> Result<(), Box<dyn std::error::Error>> {
    let models = Arc::new(models);
    
    let router = Router::new()
        .route("/query", post(handle_query))
        .route("/models", get(handle_list_models))
        .with_state(models);

    let address = "0.0.0.0:3000".parse::<std::net::SocketAddr>()?;
    println!("Starting API server on http://localhost:3000");
    axum::serve(
        tokio::net::TcpListener::bind(address).await?, 
        router
    ).await?;
    
    Ok(())
}

async fn handle_query(
    State(models): State<Arc<ModelCollection>>,
    Json(request): Json<QueryRequest>,
) -> Json<QueryResponse> {
    match models.get_model(&request.model_name) {
        Some(model) => {
            let response = model.query(&request.prompt).await
                .unwrap_or_else(|e| format!("Error: {}", e));
            Json(QueryResponse { response })
        }
        None => Json(QueryResponse { 
            response: format!("Model '{}' not found", request.model_name) 
        })
    }
}

async fn handle_list_models(
    State(models): State<Arc<ModelCollection>>,
) -> Json<Vec<ModelInfo>> {
    let model_list = models.list_models()
        .into_iter()
        .map(|(name, provider, _)| ModelInfo {
            model_name: name.clone(),
            provider: provider.to_string(),
        })
        .collect();
    
    Json(model_list)
}