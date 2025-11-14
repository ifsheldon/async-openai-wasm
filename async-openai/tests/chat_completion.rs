use async_openai_wasm::Client;
use async_openai_wasm::config::OpenAIConfig;
use async_openai_wasm::types::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use futures::StreamExt;
use serde_json::json;

const OPENROUTER_REASONING_KEY: &str = "reasoning";
const OPENROUTER_BASEURL: &str = "https://openrouter.ai/api/v1";
const DEEPSEEK_REASONING_KEY: &str = "reasoning_content";
const DEEPSEEK_BASEURL: &str = "https://api.deepseek.com";

#[tokio::test]
async fn test_chat_completion_reasoning() {
    let test_key = std::env::var("TEST_API_KEY").unwrap();
    let use_deepseek = std::env::var("USE_DEEPSEEK").is_ok();
    let (reasoning_key, base_url) = if use_deepseek {
        (DEEPSEEK_REASONING_KEY, DEEPSEEK_BASEURL)
    } else {
        (OPENROUTER_REASONING_KEY, OPENROUTER_BASEURL)
    };
    let client = Client::with_config(
        OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(test_key),
    );
    let request = CreateChatCompletionRequestArgs::default()
        .messages(vec![
            ChatCompletionRequestUserMessageArgs::default()
                .content("Hello! Do you know the Rust programming language?")
                .build()
                .unwrap()
                .into(),
        ])
        .model("deepseek/deepseek-r1")
        // The extra params that OpenRouter requires to get reasoning content
        // See https://openrouter.ai/docs/api-reference/parameters#include-reasoning
        .extra_params(json!({
            "include_reasoning" : true
        }))
        .build()
        .unwrap();
    let result = client.chat().create(request).await.unwrap();
    // Get the reasoning field in the response
    let catch_all_result = result.choices[0].message.return_catchall.as_ref().unwrap();
    let reasoning = catch_all_result
        .get(reasoning_key)
        .unwrap()
        .as_str()
        .unwrap();
    assert!(reasoning.len() > 0);
    println!("Reasoning: {reasoning}");
}

#[tokio::test]
async fn test_chat_completion_reasoning_stream() {
    let test_key = std::env::var("TEST_API_KEY").unwrap();
    let use_deepseek = std::env::var("USE_DEEPSEEK").is_ok();
    let (reasoning_key, base_url) = if use_deepseek {
        (DEEPSEEK_REASONING_KEY, DEEPSEEK_BASEURL)
    } else {
        (OPENROUTER_REASONING_KEY, OPENROUTER_BASEURL)
    };
    let client = Client::with_config(
        OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(test_key),
    );
    let request = CreateChatCompletionRequestArgs::default()
        .messages(vec![
            ChatCompletionRequestUserMessageArgs::default()
                .content("Hello! Do you know the Rust programming language?")
                .build()
                .unwrap()
                .into(),
        ])
        .model("deepseek/deepseek-r1")
        // The extra params that OpenRouter requires to get reasoning content
        // See https://openrouter.ai/docs/api-reference/parameters#include-reasoning
        .extra_params(json!({
            "include_reasoning" : true
        }))
        .build()
        .unwrap();

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
