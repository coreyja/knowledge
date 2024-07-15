use axum::{routing::any, Json, Router};
use cja::{server::run_server, setup::setup_tracing};

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing("openai_fixture")?;

    let app = Router::new().route("/v1/chat/completions", any(mock_chat_completions));

    run_server(app).await?;

    Ok(())
}

async fn mock_chat_completions() -> Json<ChatCompletionResponse> {
    let example = ChatCompletionResponse {
        id: "chatcmpl-123".to_string(),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp(),
        model: "gpt-3.5-turbo-0125".to_string(),
        system_fingerprint: "fp_44709d6fcb".to_string(),
        choices: vec![ChatCompletionChoice {
            index: 0,
            message: ChatCompletionMessage {
                role: "assistant".to_string(),
                content: "\n\nHello there, how may I assist you today?".to_string(),
            },
            logprobs: None,
            finish_reason: "stop".to_string(),
        }],
        usage: ChatCompletionUsage {
            prompt_tokens: 9,
            completion_tokens: 12,
            total_tokens: 21,
        },
    };
    Json(example)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionUsage,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessage,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ChatCompletionUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}
