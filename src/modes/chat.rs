use std::io::{self, Write};
use crate::llms::{LlmModel, model_collection::ModelCollection};

pub async fn run(models: ModelCollection) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_model: Option<&Box<dyn LlmModel>> = None;
    
    println!("Chat mode started. Available commands:");
    println!("  /list              - List all available models");
    println!("  /select <name>     - Select a model by name");
    println!("  /exit              - Exit the chat");
    println!();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            "/exit" => break,
            
            "/list" => {
                println!("\nAvailable models:");
                for (name, provider, model_name) in models.list_models() {
                    println!("- {} ({} / {})", name, provider, model_name);
                }
                println!();
            }
            
            input if input.starts_with("/select ") => {
                let model_name = input.trim_start_matches("/select ").trim();
                match models.get_model(model_name) {
                    Some(model) => {
                        current_model = Some(model);
                        println!("\nSelected model: {} ({})\n", 
                            model.model_name(), model.provider());
                    }
                    None => println!("\nModel '{}' not found. Use /list to see available models.\n", 
                        model_name),
                }
            }
            
            _ => {
                match current_model {
                    Some(model) => {
                        let response = model.query(input).await?;
                        println!("\n{}\n", response);
                    }
                    None => println!("\nPlease select a model first using /select <name>\n"),
                }
            }
        }
    }
    
    Ok(())
}