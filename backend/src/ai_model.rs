use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::process::Command;
use std::env;
use std::error::Error;

/// Gets the full path to the AI commit config file
fn get_config_path() -> String {
    env::var("HOME").map_or_else(
        |_| "/home/unknown/.ai_commit_config".to_string(),
        |home| format!("{}/.ai_commit_config", home),
    )
}

/// Reads the selected AI model from the config file
pub fn get_selected_model() -> String {
    let config_path = get_config_path();
    if let Ok(contents) = fs::read_to_string(config_path) {
        for line in contents.lines() {
            if line.starts_with("MODEL=") {
                return line.replace("MODEL=", "").trim().to_string();
            }
        }
    }
    "mistral".to_string() // Default to Mistral
}

/// Reads the OpenAI API key from the config file (if applicable)
fn get_openai_api_key() -> Result<Option<String>, Box<dyn Error>> {
    let config_path = get_config_path();
    if let Ok(contents) = fs::read_to_string(config_path) {
        for line in contents.lines() {
            if line.starts_with("OPENAI_API_KEY=") {
                return Ok(Some(line.replace("OPENAI_API_KEY=", "").trim().to_string()));
            }
        }
    }
    Ok(None)
}

/// Generates a commit message based on the AI model selected
pub fn generate_commit_message(
    changed_files: &str,
    file_diffs: &str,
    model: &str,
) -> Result<String, Box<dyn Error>> {
    if model == "openai" {
        if let Some(api_key) = get_openai_api_key()? {
            if let Ok(response) = openai_commit_message(changed_files, file_diffs, &api_key) {
                return Ok(response);
            }
        }
        println!("⚠️ OpenAI API key is missing or invalid. Falling back to Ollama (Mistral 7B)...");
    }

    // Default to using Ollama (Mistral)
    let ollama_response = ollama_commit_message(changed_files, file_diffs)?;
    Ok(ollama_response)
}

/// Uses OpenAI API to generate a commit message
fn openai_commit_message(
    changed_files: &str,
    file_diffs: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let prompt = format!(
        "Generate a concise Git commit message based on these changed files:\n{}\nAnd their changes:\n{}",
        changed_files, file_diffs
    );

    let body = json!({
        "model": "gpt-4",
        "messages": [
            { "role": "system", "content": "You are an AI that generates concise, professional Git commit messages." },
            { "role": "user", "content": prompt }
        ]
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?
        .text()?;

    Ok(response)
}

// Uses Ollama (Mistral 7B) to generate commit messages offline
fn ollama_commit_message(changed_files: &str, file_diffs: &str) -> Result<String, Box<dyn Error>> {
  let prompt = format!(
      "Generate a concise Git commit message based on these changed files:\n{}\nAnd their changes:\n{}",
      changed_files, file_diffs
  );

  if Command::new("which")
      .arg("ollama")
      .output()
      .map(|o| o.status.success())
      .unwrap_or(false)
  {
      let output = Command::new("ollama")
          .args(["run", "mistral", &prompt])
          .output()?;

      let response = String::from_utf8_lossy(&output.stdout).trim().to_string();
      if response.is_empty() {
          Err("❌ Ollama failed to generate a commit message".into())
      } else {
          Ok(response)
      }
  } else {
      println!("🧠 Installing Ollama for offline AI processing...");
      Command::new("sh")
          .arg("-c")
          .arg("curl -fsSL https://ollama.com/install.sh | sh")
          .status()?;
      Err("Ollama was not installed. Please try running the command again.".into())
  }
}

