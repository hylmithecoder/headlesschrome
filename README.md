# 👻 Screenshot Ghaib (Headless Chrome Automation Engine)

A high-performance, modular Rust engine for headless Google Chrome browser automation. It supports automated **REST API JWT Authentication**, **Interactive DOM Element Inspection**, **Step-by-Step Action Pipelines**, **Full-Page Screenshotting**, and **Animated Video Demo Recording (`.gif`)**.

---

## 🌟 Key Features

- 👻 **Invisible / Headless Chrome**: Runs Google Chrome in invisible background mode (`/usr/bin/google-chrome` auto-detected).
- 🔑 **Automated JWT API Authentication**: Authenticates with backend REST endpoints, injects `localStorage` session state, and handles React form input state setters.
- 🔍 **Interactive DOM Inspector**: Discovers buttons, input fields, links, forms, placeholders, and CSS selectors dynamically on target pages.
- ⚡ **Flexible Action Pipelines**: Supports step-by-step action sequences:
  - `inspect_dom`: Dumps interactive page elements and selectors.
  - `click`: Clicks elements by CSS selector.
  - `click_text`: Clicks buttons or links by text content.
  - `set_active_tab`: Dynamically updates application tab state in `localStorage`.
  - `type`: Types text into form fields.
  - `wait_ms`: Pauses execution for custom delays.
  - `eval_js`: Evaluates custom JavaScript code.
  - `hide_element`: Hides elements dynamically (`display: none`).
- 🎬 **Animated GIF / Video Demo Recorder**: Captures frame streams before and after action interactions and encodes them into smooth animated `.gif` video files (`recordings/`).
- 📋 **Batch Module Automation**: Configure multiple routes, URLs, and action sequences in a single `modules.json` config file.
- 🤖 **AI & Agent Ready (MCP Format)**: Built-in `--help` output providing Model Context Protocol (MCP) tool specifications and JSON schemas for AI coding assistants.

---

## 🏗️ Architecture Directory Structure

```text
screenshotghaib/
├── .cargo/
│   └── config.toml      # NixOS / Linux target linker flags (-fuse-ld=bfd)
├── src/
│   ├── main.rs          # Entry point and module orchestration loop
│   ├── config.rs        # AppConfig, AuthConfig, and ActionStep types
│   ├── browser.rs       # Chrome binary launcher & tab initializer
│   ├── auth.rs          # API JWT token retrieval & localStorage session setup
│   ├── dom.rs           # Step-by-step action pipeline execution engine
│   ├── jsparsing.rs     # Interactive DOM element scanner & inspector
│   └── recorder.rs      # Frame recorder & animated GIF video encoder
├── modules.json         # Module configuration & action pipeline definitions
├── Cargo.toml           # Project dependencies (headless_chrome, serde, image)
└── Makefile             # Command shortcut targets (run, build, check, test, clean)
```

---

## 🚀 Quick Start

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/) (Installed via Rustup)
- Google Chrome or Chromium installed (`/usr/bin/google-chrome`)

### 🛠️ Using `make` Shortcuts

```bash
# Run batch modules defined in modules.json
make run

# Check project compilation
make check

# Build optimized release binary
make build

# Display all available make targets
make help
```

### 💻 Using `cargo` Directly

```bash
# Display help and AI-readable tool specification
cargo run -- --help

# Run default batch configuration (modules.json)
cargo run

# Run custom JSON configuration file
cargo run my_modules.json

# Single URL screenshot mode
cargo run "https://youtube.com" "youtube_screenshot.png"
```

---

## 📋 JSON Configuration Schema (`modules.json`)

```json
{
  "base_url": "http://localhost:3000",
  "auth": {
    "login_url": "http://localhost:3000/login",
    "api_url": "http://127.0.0.1:5700/api/v1/auth/login",
    "token_key": "siabsen-token",
    "user_key": "siabsen-user",
    "username": "admin",
    "password": "admin123"
  },
  "modules": [
    {
      "name": "01_login_page",
      "url": "http://localhost:3000/login",
      "output": "01_login_page.png",
      "requires_auth": false,
      "actions": [
        { "type": "inspect_dom" }
      ]
    },
    {
      "name": "02_dashboard_ringkasan",
      "url": "http://localhost:3000/dashboard",
      "output": "02_dashboard_ringkasan.png",
      "requires_auth": true,
      "record_video": true,
      "video_output": "02_dashboard_demo.gif",
      "actions": [
        { "type": "click_text", "text": "Ringkasan" },
        { "type": "wait_ms", "duration": 1000 }
      ]
    },
    {
      "name": "03_data_siswa_guru",
      "url": "http://localhost:3000/dashboard",
      "output": "03_data_siswa_guru.png",
      "requires_auth": true,
      "record_video": true,
      "video_output": "03_data_siswa_demo.gif",
      "actions": [
        { "type": "set_active_tab", "tab_id": "data" },
        { "type": "wait_ms", "duration": 1500 }
      ]
    }
  ]
}
```

---

## 📂 Output Directories

- **`screenshots/`**: Full-page PNG screenshots saved per module.
- **`recordings/`**: Animated GIF video recordings produced during action pipeline interactions.

---

## 📄 License

Created by [Hylmi Mahdi](https://github.com/hylmithecoder).
