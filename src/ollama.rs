use serde::{Deserialize, Serialize};
use reqwest::Client;

const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn get_command_from_prompt(prompt: &str, distro_id: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let system_prompt = format!(
        "You are an AI assistant running on Linux created by CoderFetch21. The user is running the '{}' distribution and you are to help with terminal commands. \
        Convert the following natural language command into a single, executable shell command for that environment. \
        Only output the shell command itself, with no additional explanation or formatting. \
        For example, if the user says 'list the files', you should only output 'ls -la'.",
        distro_id
    );

    let full_prompt = format!("{} \n\nUser: {}", system_prompt, prompt);

    let request = OllamaRequest {
        model: "llama3.2",
        prompt: full_prompt,
        stream: false,
    };

    let res = client.post(OLLAMA_API_URL)
        .json(&request)
        .send()
        .await?;

    let response_json: OllamaResponse = res.json().await?;
    
    // The model might still add quotes or newlines, so we clean it up.
    let command = response_json.response.trim().trim_matches('`').trim().to_string();

    Ok(command)
}
