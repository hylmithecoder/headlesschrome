mod auth;
mod browser;
mod config;
mod dom;
mod jsparsing;
mod recorder;

use auth::perform_login;
use browser::init_browser;
use config::AppConfig;
use dom::execute_action_steps;
use jsparsing::inspect_dom_elements;
use recorder::FrameRecorder;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }

    let config_path = if args.len() > 1 && args[1].ends_with(".json") {
        args[1].clone()
    } else {
        "modules.json".to_string()
    };

    println!("👻 [Ghaib Engine] Starting Modular Headless Screenshot & Video Engine...");

    let screenshot_dir = Path::new("screenshots");
    if !screenshot_dir.exists() {
        fs::create_dir_all(screenshot_dir)?;
    }

    let recording_dir = Path::new("recordings");
    if !recording_dir.exists() {
        fs::create_dir_all(recording_dir)?;
    }

    let browser = init_browser()?;
    let tab = browser.new_tab()?;

    if Path::new(&config_path).exists() {
        let config_str = fs::read_to_string(&config_path)?;
        let config: AppConfig = serde_json::from_str(&config_str)?;

        println!(
            "📋 Loaded configuration with {} modules.",
            config.modules.len()
        );

        let mut is_logged_in = false;

        for (idx, module) in config.modules.iter().enumerate() {
            println!("\n========================================");
            println!(
                "🚀 [{}/{}] Module: {}",
                idx + 1,
                config.modules.len(),
                module.name
            );

            // Initialize video recorder if enabled for this module
            let mut video_recorder = if module.record_video.unwrap_or(false) {
                println!(
                    "🎥 Interactive video recording enabled for module '{}'!",
                    module.name
                );
                Some(FrameRecorder::new())
            } else {
                None
            };

            // 1. Authenticate if required
            if module.requires_auth && !is_logged_in {
                println!("🔒 Auth required. Executing modular auto-login...");
                perform_login(&tab, &config.auth)?;
                is_logged_in = true;
            }

            // 2. Navigate to target URL
            println!("🌐 Navigating to: {}", module.url);
            tab.navigate_to(&module.url)?;
            tab.wait_until_navigated()?;
            std::thread::sleep(Duration::from_secs(2));

            // Record initial frame
            if let Some(ref mut rec) = video_recorder {
                let _ = rec.capture_frame(&tab);
            }

            // 3. Inspect DOM elements
            inspect_dom_elements(&tab)?;

            // 4. Run flexible action steps pipeline with video frame recording
            if let Some(actions) = &module.actions {
                println!("⚡ Running action steps pipeline...");
                execute_action_steps(&tab, actions, video_recorder.as_mut())?;
            }

            // 5. Legacy js_before fallback
            if let Some(js_code) = &module.js_before {
                if module.actions.is_none() {
                    println!("⚡ Executing legacy JS snippet: {}", js_code);
                    let _ = tab.evaluate(js_code, false);
                    std::thread::sleep(Duration::from_secs(1));
                    if let Some(ref mut rec) = video_recorder {
                        let _ = rec.capture_frame(&tab);
                    }
                }
            }

            // 6. Capture PNG screenshot
            println!("📸 Capturing final screenshot...");
            let png_bytes = tab.capture_screenshot(
                headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
                None,
                None,
                true,
            )?;

            let screenshot_path = screenshot_dir.join(&module.output);
            fs::write(&screenshot_path, png_bytes)?;
            println!("✅ Saved screenshot to: {}", screenshot_path.display());

            // 7. Save recorded video demo if enabled
            if let Some(ref rec) = video_recorder {
                let video_filename = module
                    .video_output
                    .clone()
                    .unwrap_or_else(|| format!("{}_demo.gif", module.name));
                let video_path = recording_dir.join(&video_filename);
                rec.save_animated_gif(&video_path, 800)?;
            }
        }
    } else {
        let url = if args.len() > 1 {
            &args[1]
        } else {
            "https://www.youtube.com"
        };
        let output_file = if args.len() > 2 {
            &args[2]
        } else {
            "screenshot.png"
        };

        println!("🌐 Single URL mode -> Target: {}", url);
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;
        std::thread::sleep(Duration::from_secs(3));

        inspect_dom_elements(&tab)?;

        let png_bytes = tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )?;

        fs::write(output_file, png_bytes)?;
        println!("✨ Screenshot saved to: {}", output_file);
    }

    println!("\n🎉 All screenshot and video recording tasks completed successfully!");
    Ok(())
}

fn print_help() {
    println!(
        r#"
👻 Screenshot Ghaib Engine - Headless Chrome Automation Tool

USAGE:
  screenshotghaib [OPTIONS] [CONFIG_PATH | URL] [OUTPUT_FILE]

ARGUMENTS:
  CONFIG_PATH   Path to JSON configuration file (Default: modules.json)
  URL           Target web address (Single URL mode, e.g. https://example.com)
  OUTPUT_FILE   Output image filename for single URL mode (Default: screenshot.png)

OPTIONS:
  -h, --help    Display this help and AI-readable tool specification

AI & AGENT TOOL SPECIFICATION (MCP Format):
  Name: screenshot_ghaib
  Description: Automates headless Google Chrome to capture full-page screenshots,
               scan DOM elements, execute action pipelines, and record animated GIF videos.

  JSON Configuration Schema (modules.json):
  {{
    "base_url": "http://localhost:3000",
    "auth": {{
      "login_url": "http://localhost:3000/login",
      "api_url": "http://127.0.0.1:5700/api/v1/auth/login",
      "token_key": "siabsen-token",
      "user_key": "siabsen-user",
      "username": "admin",
      "password": "admin123"
    }},
    "modules": [
      {{
        "name": "module_name",
        "url": "http://localhost:3000/target_route",
        "output": "screenshot_name.png",
        "requires_auth": true,
        "record_video": true,
        "video_output": "demo_video.gif",
        "actions": [
          {{ "type": "inspect_dom" }},
          {{ "type": "click", "selector": "button.submit" }},
          {{ "type": "click_text", "text": "Ringkasan" }},
          {{ "type": "set_active_tab", "tab_id": "data" }},
          {{ "type": "type", "selector": "input[name='search']", "text": "query" }},
          {{ "type": "wait_ms", "duration": 1000 }},
          {{ "type": "eval_js", "script": "console.log('hello')" }},
          {{ "type": "hide_element", "selector": ".modal" }}
        ]
      }}
    ]
  }}

EXAMPLES:
  # Execute batch configuration modules.json
  screenshotghaib

  # Execute custom JSON config file
  screenshotghaib my_modules.json

  # Single URL mode
  screenshotghaib "https://youtube.com" "youtube.png"
"#
    );
}
