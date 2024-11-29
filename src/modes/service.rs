#[cfg(windows)]
use std::{
    ffi::OsString,
    time::Duration,
    path::PathBuf,
    fs::OpenOptions,
};

#[cfg(windows)]
use log::{info, error, warn};
#[cfg(windows)]
use simplelog::{WriteLogger, LevelFilter, Config};
#[cfg(windows)]
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};
#[cfg(windows)]
use crate::llms::model_collection::ModelCollection;

#[cfg(windows)]
const SERVICE_NAME: &str = "LlmApiService";
#[cfg(windows)]
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

#[cfg(windows)]
static mut SERVICE_PORT: u16 = 3000; // Default port

#[cfg(windows)]
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let log_path = exe_path.parent().unwrap_or(&PathBuf::from(".")).join("llm-api.log");
    
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(log_path.clone())?;

    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        log_file,
    )?;
    
    info!("Logging initialized at {}", log_path.display());
    Ok(())
}

#[cfg(windows)]
pub fn run(port: u16) -> windows_service::Result<()> {
    unsafe {
        SERVICE_PORT = port;
    }
    
    if let Err(e) = setup_logging() {
        error!("Failed to initialize logging: {}", e);
        eprintln!("Failed to initialize logging: {}", e);
        let error = windows_service::Error::LaunchArgumentsNotSupported;
        return Err(error);
    }
    
    info!("Starting {} on port {}", SERVICE_NAME, port);
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}

#[cfg(windows)]
define_windows_service!(ffi_service_main, service_main);

#[cfg(windows)]
fn service_main(arguments: Vec<OsString>) {
    info!("Service main started with {:?} arguments", arguments);
    if let Err(e) = run_service(arguments) {
        error!("Service error: {}", e);
        // Keep eprintln for Windows Event Log visibility
        eprintln!("Service error: {}", e);
    }
}

#[cfg(windows)]
fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    info!("Initializing service control handler");
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                info!("Service stop requested");
                ServiceControlHandlerResult::NoError
            }
            _ => {
                warn!("Received unsupported service control command");
                ServiceControlHandlerResult::NotImplemented
            }
        }
    };

    let status_handle = match service_control_handler::register(SERVICE_NAME, event_handler) {
        Ok(handle) => {
            info!("Service control handler registered");
            handle
        }
        Err(e) => {
            error!("Failed to register service control handler: {}", e);
            return Err(windows_service::Error::LaunchArgumentsNotSupported);
        }
    };

    if let Err(e) = status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }) {
        error!("Failed to set service status: {}", e);
        return Err(windows_service::Error::LaunchArgumentsNotSupported);
    }
    info!("Service status set to running");

    // Start the API server with the specified port
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            error!("Failed to create Tokio runtime: {}", e);
            return Err(windows_service::Error::LaunchArgumentsNotSupported);
        }
    };
    
    let models = ModelCollection::new();
    let port = unsafe { SERVICE_PORT };
    
    info!("Starting API server on port {}", port);
    runtime.block_on(async {
        if let Err(e) = crate::modes::api::run(models, port).await {
            error!("API server error: {}", e);
            eprintln!("API server error: {}", e);
        }
    });

    info!("Service shutting down");
    Ok(())
}
