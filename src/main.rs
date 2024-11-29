mod llms;
mod modes;
mod config;

use clap::{Parser, ValueEnum};
use llms::model_collection::ModelCollection;
use config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mode to run in (chat or api)
    #[arg(value_enum)]
    mode: Mode,

    /// Set the port number for the API server
    #[arg(long = "set-port")]
    port: Option<u16>,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Chat,
    Api,
    #[cfg(windows)]
    Service,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Handle port configuration if specified
    if let Some(port) = cli.port {
        let config = Config { port };
        config.save()?;
        println!("Port configuration saved. API will now use port {}", port);
        return Ok(());
    }
    
    let models = ModelCollection::new();
    let config = Config::load();
    
    match cli.mode {
        Mode::Chat => modes::chat::run(models).await?,
        Mode::Api => modes::api::run(models, config.port).await?,
        #[cfg(windows)]
        Mode::Service => modes::service::run()?,
    }
    
    Ok(())
}
