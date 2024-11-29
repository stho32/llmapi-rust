mod llms;
mod modes;

use clap::{Parser, ValueEnum};
use llms::model_collection::ModelCollection;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mode to run in (chat or api)
    #[arg(value_enum)]
    mode: Mode
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Chat,
    Api,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    let models = ModelCollection::new();
    
    match cli.mode {
        Mode::Chat => modes::chat::run(models).await?,
        Mode::Api => modes::api::run(models).await?,
    }
    
    Ok(())
}
