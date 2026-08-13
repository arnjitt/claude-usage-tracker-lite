// Claude Usage Tracker — Tauri shell.
// The OAuth token is read fresh from ~/.claude/.credentials.json on every
// refresh and never leaves this process: the renderer only ever sees the
// usage JSON. The endpoint is the one Claude Code's own /usage screen uses;
// it is unofficial, so every failure maps to an explicit error string — the
// app degrades to an error state, never to wrong numbers.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

const USAGE_URL: &str = "https://api.anthropic.com/api/oauth/usage";

fn read_token() -> Result<String, String> {
    let home = dirs::home_dir().ok_or("no-creds")?;
    let path = home.join(".claude").join(".credentials.json");
    let raw = std::fs::read_to_string(path).map_err(|_| "no-creds")?;
    let creds: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "no-creds")?;
    creds["claudeAiOauth"]["accessToken"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "no-creds".into())
}

fn fetch_usage_blocking() -> Result<serde_json::Value, String> {
    let token = read_token()?;
    let resp = ureq::get(USAGE_URL)
        .set("Authorization", &format!("Bearer {token}"))
        .set("anthropic-beta", "oauth-2025-04-20")
        .timeout(Duration::from_secs(10))
        .call();
    match resp {
        Ok(r) => r.into_json().map_err(|_| "bad-json".into()),
        Err(ureq::Error::Status(401, _)) => Err("auth".into()),
        Err(ureq::Error::Status(code, _)) => Err(format!("http-{code}")),
        Err(_) => Err("offline".into()),
    }
}

#[tauri::command]
async fn fetch_usage() -> Result<serde_json::Value, String> {
    tauri::async_runtime::spawn_blocking(fetch_usage_blocking)
        .await
        .map_err(|_| String::from("internal"))?
}

#[tauri::command]
fn set_pin(window: tauri::WebviewWindow, pinned: bool) -> Result<(), String> {
    window.set_always_on_top(pinned).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_usage, set_pin])
        .run(tauri::generate_context!())
        .expect("error while running claude-usage-tracker");
}
