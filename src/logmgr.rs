use super::logger; 

pub fn log_manager(message: &str) {
    #[cfg(target_os = "linux")]
    let path = "~/.R-touch/logs/r-touch.log";

    #[cfg(target_os = "windows")]
    let path = "~\\Desktop\\R-touch\\logs";

    if let Err(e) = logger::Logger::log(path, &message) {
        eprintln!("Error logging the action. Error: {e}");
        std::process::exit(1);
    }
}
