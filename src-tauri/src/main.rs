// Claude Usage Tracker — Tauri shell.
// The OAuth token is read fresh from Claude Code's own credential store on
// every refresh and never leaves this process: the renderer only ever sees
// the usage JSON. The endpoint is the one Claude Code's own /usage screen
// uses; it is unofficial, so every failure maps to an explicit error string —
// the app degrades to an error state, never to wrong numbers.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

const USAGE_URL: &str = "https://api.anthropic.com/api/oauth/usage";

const WINDOW_WIDTH: f64 = 440.0;

fn token_from_json(raw: &str) -> Option<String> {
    let creds: serde_json::Value = serde_json::from_str(raw).ok()?;
    creds["claudeAiOauth"]["accessToken"]
        .as_str()
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
}

fn file_token() -> Option<String> {
    let path = dirs::home_dir()?.join(".claude").join(".credentials.json");
    token_from_json(&std::fs::read_to_string(path).ok()?)
}

// On macOS Claude Code keeps its credentials in the login Keychain, not in a
// file. It writes the item with the `security` tool, so the same tool can
// read it back without a prompt.
#[cfg(target_os = "macos")]
fn keychain_token() -> Option<String> {
    let out = std::process::Command::new("/usr/bin/security")
        .args(["find-generic-password", "-s", "Claude Code-credentials", "-w"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    token_from_json(String::from_utf8(out.stdout).ok()?.trim())
}

fn read_token() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    if let Some(token) = keychain_token() {
        return Ok(token);
    }
    file_token().ok_or_else(|| "no-creds".into())
}

// Retry-After as whole seconds; the HTTP-date form is ignored (the UI then
// falls back to its own default delay).
fn retry_after_secs(header: Option<&str>) -> Option<u64> {
    header?.trim().parse().ok()
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
        Err(ureq::Error::Status(429, r)) => Err(match retry_after_secs(r.header("retry-after")) {
            Some(secs) => format!("http-429:{secs}"),
            None => "http-429".into(),
        }),
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

// The renderer reports its content height so the window always fits what the
// server sent — two meters or five, with or without an error banner.
#[tauri::command]
fn set_height(window: tauri::WebviewWindow, height: f64) -> Result<(), String> {
    if !height.is_finite() {
        return Err("bad-height".into());
    }
    window
        .set_size(tauri::LogicalSize::new(WINDOW_WIDTH, height.clamp(120.0, 900.0)))
        .map_err(|e| e.to_string())
}

// Debug builds only: `cargo run -- --dump` prints the raw usage reply the app
// would render and exits, for checking what the unofficial endpoint sends.
#[cfg(debug_assertions)]
fn dump_if_asked() {
    if std::env::args().nth(1).as_deref() != Some("--dump") {
        return;
    }
    match fetch_usage_blocking() {
        Ok(body) => println!("{}", serde_json::to_string_pretty(&body).unwrap_or_default()),
        Err(e) => println!("error: {e}"),
    }
    std::process::exit(0);
}

fn main() {
    #[cfg(debug_assertions)]
    dump_if_asked();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_usage, set_pin, set_height])
        .run(tauri::generate_context!())
        .expect("error while running claude-usage-tracker");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_read_from_the_oauth_block() {
        let raw = r#"{"claudeAiOauth":{"accessToken":"abc","expiresAt":1}}"#;
        assert_eq!(token_from_json(raw).as_deref(), Some("abc"));
    }

    #[test]
    fn missing_empty_or_malformed_credentials_yield_no_token() {
        assert_eq!(token_from_json("{}"), None);
        assert_eq!(token_from_json(r#"{"claudeAiOauth":{"accessToken":""}}"#), None);
        assert_eq!(token_from_json(r#"{"claudeAiOauth":{"accessToken":5}}"#), None);
        assert_eq!(token_from_json("not json"), None);
    }

    #[test]
    fn retry_after_accepts_only_whole_seconds() {
        assert_eq!(retry_after_secs(Some("120")), Some(120));
        assert_eq!(retry_after_secs(Some(" 45 ")), Some(45));
        assert_eq!(retry_after_secs(Some("Wed, 21 Oct 2026 07:28:00 GMT")), None);
        assert_eq!(retry_after_secs(None), None);
    }
}
