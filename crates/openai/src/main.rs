use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let secret_key = std::env::var("OPENAI_API_KEY").unwrap();
    let model_name = std::env::var("OPENAI_MODEL_NAME").unwrap();

    let config = OpenAIConfig::default()
        .with_api_base("https://openrouter.ai/api/v1")
        .with_api_key(secret_key);

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1024u16)
        .model(model_name)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Hello there!")
                .build()?
                .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}
