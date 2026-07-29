use headless_chrome::{Browser, LaunchOptions};
use std::path::{Path, PathBuf};

pub fn init_browser() -> Result<Browser, Box<dyn std::error::Error>> {
    let chrome_binary = if Path::new("/usr/bin/google-chrome").exists() {
        Some(PathBuf::from("/usr/bin/google-chrome"))
    } else {
        None
    };

    let options = LaunchOptions::default_builder()
        .headless(true)
        .path(chrome_binary)
        .window_size(Some((1920, 1080)))
        .build()
        .map_err(|e| format!("Failed to build Chrome options: {}", e))?;

    let browser = Browser::new(options)?;
    Ok(browser)
}
