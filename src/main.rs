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
    #[arg(long = "port")]
    port: Option<u16>,

    /// Set the port number in config file
    #[arg(long = "set-port")]
    set_port: Option<u16>,
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
    if let Some(port) = cli.set_port {
        let config = Config { port };
        config.save()?;
        println!("Port configuration saved. API will now use port {}", port);
        return Ok(());
    }
    
    let models = ModelCollection::new();
    let config = Config::load();
    
    // Use CLI port if specified, otherwise use config port
    let port = cli.port.unwrap_or(config.port);
    
    match cli.mode {
        Mode::Chat => modes::chat::run(models).await?,
        Mode::Api => modes::api::run(models, port).await?,
        #[cfg(windows)]
        Mode::Service => modes::service::run(port)?,
    }
    
    Ok(())
}
