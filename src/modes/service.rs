#[cfg(windows)]
use std::{
    ffi::OsString,
    time::Duration,
};
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
pub fn run(port: u16) -> windows_service::Result<()> {
    unsafe {
        SERVICE_PORT = port;
    }
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}

#[cfg(windows)]
define_windows_service!(ffi_service_main, service_main);

#[cfg(windows)]
fn service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        // TODO: Implement proper error logging
        eprintln!("Service error: {}", e);
    }
}

#[cfg(windows)]
fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                // TODO: Implement proper cleanup
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // Start the API server with the specified port
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let models = ModelCollection::new();
    let port = unsafe { SERVICE_PORT };
    
    runtime.block_on(async {
        if let Err(e) = crate::modes::api::run(models, port).await {
            eprintln!("Service error: {}", e);
        }
    });

    Ok(())
}
