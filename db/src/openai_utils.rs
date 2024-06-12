use color_eyre::Result;
use openai::chat::{ChatCompletionBuilder, ChatCompletionMessage, ChatCompletionMessageRole};
use std::env;

pub async fn generate_summary(content: &str) -> Result<String> {
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
    openai::set_key(api_key);

    let messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(format!("Provide a concise summary of the following article that can be read and understood in 2 minutes, focusing on the main points and essential details: {content}")),
            name: None,
            function_call: None,
        },
    ];
    let request = ChatCompletionBuilder::default()
        .model("gpt-4o".to_string())
        .messages(messages)
        .max_tokens(4096u64)
        .temperature(0.7)
        .build()?;

    let response = openai::chat::ChatCompletion::create(&request).await?;
    let summary = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default();
    Ok(summary)
}
