use std::{
    ffi::OsString,
    time::Duration,
};
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};
use crate::llms::model_collection::ModelCollection;

const SERVICE_NAME: &str = "LlmApiService";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn run() -> windows_service::Result<()> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        // TODO: Implement proper error logging
        eprintln!("Service error: {}", e);
    }
}

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

    // Start API server in service context
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let models = ModelCollection::new();
        let config = crate::config::Config::load();
        if let Err(e) = crate::modes::api::run(models, config.port).await {
            eprintln!("API server error: {}", e);
        }
    });

    Ok(())
}
