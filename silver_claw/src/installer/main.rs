use std::ffi::OsString;
use std::path::PathBuf;
use windows_service::service::{
    ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType,
};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

fn main() -> windows_service::Result<()> {
    println!("Installing silver_claw service");

    let manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)?;

    let service_info = ServiceInfo {
        name: OsString::from("silver_claw"),
        display_name: OsString::from("silver_claw"),

        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: PathBuf::from("C:/Users/mremond/Documents/DevProjects/Rust/silver_claw_project/silver_claw/target/debug/silver_claw.exe"),
        launch_arguments: vec![],
        dependencies: vec![],

        account_name: None, // Run as System
        account_password: None,
    };

    let _service = manager.create_service(&service_info, ServiceAccess::QUERY_STATUS)?;

    println!("Installation succeeded");

    Ok(())
}
