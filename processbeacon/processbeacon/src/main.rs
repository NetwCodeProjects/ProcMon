use sysinfo::{ProcessExt, System, SystemExt};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use hostname::get;
use windows_service::{
    define_windows_service, service::*, service_control_handler::{self, ServiceControlHandlerResult}, service_dispatcher,
};

define_windows_service!(ffi_service_main, processbeacon_service_main);

fn processbeacon_service_main(_arguments: Vec<std::ffi::OsString>) {
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let status_handle = service_control_handler::register("processbeacon", move |control_event| {
        if let ServiceControl::Stop = control_event {
            shutdown_tx.send(()).unwrap();
        }
        ServiceControlHandlerResult::NoError
    })
    .unwrap();

    status_handle
        .set_service_status(ServiceStatus {
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            service_type: ServiceType::OWN_PROCESS,
            process_id: None,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(0),
        })
        .unwrap();

    let mut system = System::new_all();
    let hostname = get()
        .unwrap_or_else(|_| "Unknown Host".into())
        .to_string_lossy()
        .to_string();

    let log_file_path = get_log_file_path();
    write_to_log(&log_file_path, "processbeacon started and logging initialized.");

    let mut already_notified = false;

    loop {
        if shutdown_rx.recv_timeout(Duration::from_secs(5)).is_ok() {
            break;
        }

        // Refresh process list
        system.refresh_processes();

        // Check if the app is running (monitor explorer.exe as an example)
        let app_running = system
            .processes()
            .iter()
            .any(|(_, process)| process.name() == "Taskmgr.exe");

        if app_running && !already_notified {
            let message = format!(
                "{} has logged in at {}",
                hostname,
                get_current_time()
            );
            write_to_log(&log_file_path, &message);
            already_notified = true; // Prevent repeated notifications until it stops running
        }

        if !app_running && already_notified {
            already_notified = false; // Reset the notification state
        }
    }

    status_handle
        .set_service_status(ServiceStatus {
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            service_type: ServiceType::OWN_PROCESS,
            process_id: None,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(0),
        })
        .unwrap();
}

fn get_current_time() -> String {
    let now = SystemTime::now();
    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(_) => format!("{:?}", chrono::Local::now()),
        Err(_) => "Unknown Time".to_string(),
    }
}

fn write_to_log(file_path: &str, message: &str) {
    let path = Path::new(file_path);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!(
                    "[{}] Failed to create directory {}: {}",
                    get_current_time(),
                    parent.display(),
                    e
                );
                return;
            }
        }
    }

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "[{}] Failed to open or create log file {}: {}",
                get_current_time(),
                file_path,
                e
            );
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!(
            "[{}] Failed to write to log file {}: {}",
            get_current_time(),
            file_path,
            e
        );
    }
}

fn get_log_file_path() -> String {
    let base_path = "C:\\ProgramData\\processbeacon";
    let log_file_path = format!("{}\\processbeacon.log", base_path);

    if !Path::new(base_path).exists() {
        if let Err(e) = fs::create_dir_all(base_path) {
            eprintln!(
                "[{}] Failed to create base directory {}: {}",
                get_current_time(),
                base_path,
                e
            );
            std::process::exit(1);
        }
    }

    log_file_path
}

fn main() -> Result<(), windows_service::Error> {
    service_dispatcher::start("processbeacon", ffi_service_main)
}
