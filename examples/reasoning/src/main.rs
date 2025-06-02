use async_openai_wasm::Client;
use async_openai_wasm::config::OpenAIConfig;
use async_openai_wasm::types::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use futures::StreamExt;
use serde_json::json;

const OPENROUTER_REASONING_KEY: &str = "reasoning";
const OPENROUTER_BASEURL: &str = "https://openrouter.ai/api/v1";
const OPENROUTER_MODEL_NAME: &str = "deepseek/deepseek-r1";
const DEEPSEEK_REASONING_KEY: &str = "reasoning_content";
const DEEPSEEK_BASEURL: &str = "https://api.deepseek.com";
const DEEPSEEK_MODEL_NAME: &str = "deepseek-reasoner";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let test_key = std::env::var("TEST_API_KEY").unwrap();
    let use_deepseek = std::env::var("USE_DEEPSEEK").is_ok();
    let (reasoning_key, base_url, model_name) = if use_deepseek {
        (
            DEEPSEEK_REASONING_KEY,
            DEEPSEEK_BASEURL,
            DEEPSEEK_MODEL_NAME,
        )
    } else {
        (
            OPENROUTER_REASONING_KEY,
            OPENROUTER_BASEURL,
            OPENROUTER_MODEL_NAME,
        )
    };
    let client = Client::with_config(
        OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(test_key),
    );
    let messages = vec![
        ChatCompletionRequestUserMessageArgs::default()
            .content("Hello! Do you know the Rust programming language?")
            .build()
            .unwrap()
            .into(),
    ];
    let request = if use_deepseek {
        CreateChatCompletionRequestArgs::default()
            .messages(messages)
            .model(model_name)
            .build()
            .unwrap()
    } else {
        CreateChatCompletionRequestArgs::default()
            .messages(messages)
            .model(model_name)
            // The extra params that OpenRouter requires to get reasoning content
            // See https://openrouter.ai/docs/api-reference/parameters#include-reasoning
            .extra_params(json!({
                "include_reasoning" : true
            }))
            .build()
            .unwrap()
    };

    let mut result = client.chat().create_stream(request).await.unwrap();
    let mut reasoning = String::new();

    while let Some(result) = result.next().await {
        if let Ok(r) = result {
            // Get the reasoning field in the response
            let catch_all_return = r.choices[0].delta.return_catchall.as_ref();
            let reasoning_part = catch_all_return
                .and_then(|val| val.get(reasoning_key))
                .and_then(|r| r.as_str());
            if let Some(reasoning_part) = reasoning_part {
                reasoning.push_str(reasoning_part);
                println!("Reasoning Part: {reasoning_part}")
            }
        }
    }
    assert!(reasoning.len() > 0);
    println!("Reasoning:\n{reasoning}");
}
