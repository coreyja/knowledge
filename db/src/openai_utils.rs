use color_eyre::Result;
use openai::chat::{ChatCompletionBuilder, ChatCompletionMessage, ChatCompletionMessageRole};
use std::env;

pub async fn generate_summary(content: &str) -> Result<String> {
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
    openai::set_key(api_key);

    let messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some("You are a helpful assistant.".to_string()),
            name: None,
            function_call: None,
        },
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(format!("Summarize the following article and make sure to highlight the important parts: {content}")),
            name: None,
            function_call: None,
        },
    ];
    let request = ChatCompletionBuilder::default()
        .model("gpt-4o".to_string())
        .messages(messages)
        .max_tokens(4096u64)
        .temperature(0.7)
        .top_p(1.0)
        .build()?;

    let response = openai::chat::ChatCompletion::create(&request).await?;
    let summary = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default();
    Ok(summary)
}
