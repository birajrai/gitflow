use std::time::Duration;

use serde_json::{json, Value};

const NIM_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";
const SYSTEM_PROMPT: &str = "You are an expert git assistant. Write a conventional commits style \
commit message for the given diff. The first line must be a summary under 72 characters using a \
conventional prefix (feat, fix, chore, docs, refactor, test, perf). For non-trivial changes add \
short bullet points as a body, separated from the summary by one blank line. Output only the \
commit message. Do not use markdown code fences. Do not add preamble or commentary.";

fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        s.to_string()
    } else {
        let cut: String = chars[..max].iter().collect();
        format!("{cut}…")
    }
}

async fn call_nim(api_key: &str, body: Value) -> Result<Value, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(NIM_URL)
        .bearer_auth(api_key)
        .json(&body)
        .timeout(Duration::from_secs(90))
        .send()
        .await
        .map_err(|e| format!("Request to NIM failed: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        let body_text = resp.text().await.unwrap_or_default();
        return Err(format!("NIM API error {status}: {}", truncate(&body_text, 300)));
    }

    resp.json().await.map_err(|e| format!("Invalid NIM response: {e}"))
}

#[tauri::command]
pub async fn generate_commit_message(api_key: String, model: String, diff: String) -> Result<String, String> {
    let capped: String = diff.chars().take(12_000).collect();
    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": SYSTEM_PROMPT },
            { "role": "user", "content": format!("Here is the diff:\n\n{capped}") }
        ],
        "temperature": 0.2
    });

    let resp = call_nim(&api_key, body).await?;
    resp["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.trim().to_string())
        .ok_or_else(|| String::from("NIM returned no message content"))
}

#[tauri::command]
pub async fn test_nim_connection(api_key: String, model: String) -> Result<(), String> {
    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": "Reply with the single word OK." },
            { "role": "user", "content": "Ping." }
        ],
        "max_tokens": 10,
        "temperature": 0.0
    });

    call_nim(&api_key, body).await?;
    Ok(())
}
