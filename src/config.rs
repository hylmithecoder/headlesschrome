use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub login_url: String,
    pub api_url: Option<String>,
    pub username: String,
    pub password: String,
    pub token_key: Option<String>,
    pub user_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionStep {
    InspectDom,
    Click { selector: String },
    ClickText { text: String },
    SetActiveTab { tab_id: String },
    Type { selector: String, text: String },
    WaitMs { duration: u64 },
    EvalJs { script: String },
    HideElement { selector: String },
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub url: String,
    pub output: String,
    #[serde(default)]
    pub requires_auth: bool,
    pub actions: Option<Vec<ActionStep>>,
    pub js_before: Option<String>,
    pub record_video: Option<bool>,
    pub video_output: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub base_url: String,
    pub auth: AuthConfig,
    pub modules: Vec<ModuleConfig>,
}
