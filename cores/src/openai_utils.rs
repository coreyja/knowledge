use color_eyre::Result;
use openai::chat::{ChatCompletionBuilder, ChatCompletionMessage, ChatCompletionMessageRole};
use openai::embeddings::Embeddings;

async fn generate_openai_response(prompt: &str, max_tokens: u64) -> Result<String> {
    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(prompt.to_string()),
        name: None,
        function_call: None,
    }];
    let request = ChatCompletionBuilder::default()
        .model("gpt-4o".to_string())
        .messages(messages)
        .max_tokens(max_tokens)
        .temperature(0.7)
        .build()?;

    let response = openai::chat::ChatCompletion::create(&request).await?;
    let content = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default();
    Ok(content)
}

pub async fn generate_summary(content: &str) -> Result<String> {
    let prompt = format!("Provide a concise summary of the following article that can be read and understood in 2 minutes, focusing on the main points and essential details: {content}");
    generate_openai_response(&prompt, 4096).await
}

pub async fn generate_categories(content: &str) -> Result<String> {
    let prompt = format!("Provide unique categories for the following article. Do not exceed two words per category. Separate categories with commas:
: {content}");
    let response = generate_openai_response(&prompt, 100).await?;
    let category = response
        .split(',')
        .next()
        .unwrap_or_default()
        .trim()
        .to_string();
    Ok(category)
}

pub async fn generate_embedding(text: &str) -> color_eyre::Result<Vec<f64>> {
    let embedding = Embeddings::create("text-embedding-ada-002", vec![text], "user-id").await?;
    Ok(embedding.data[0].vec.clone())
}
