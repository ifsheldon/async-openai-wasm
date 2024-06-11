#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};
use futures::stream::StreamExt;
use async_openai_wasm::Client;
use async_openai_wasm::config::OpenAIConfig;
use async_openai_wasm::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};


const API_BASE: &str = "...";
const API_KEY: &str = "...";

pub fn App() -> Element {
    const GREETING: &str = "Hello! How are you?";
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(GREETING)
                    .build()
                    .unwrap()
            )
        ])
        .build().unwrap();
    let response_string = use_signal(String::new);
    let _fetch_completion_chunks: Coroutine<()> = use_coroutine(|_rx| {
        let mut response_string = response_string.to_owned();
        async move {
            let config = OpenAIConfig::new()
                .with_api_key(API_KEY);
            let config = if API_BASE != "..." {
                config.with_api_base(API_BASE)
            } else {
                config
            };
            let client = Client::with_config(config);
            let mut stream = client.chat().create_stream(request).await.unwrap();
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(response) =>
                        response_string.with_mut(|string| {
                            if let Some(content) = response.choices[0].delta.content.as_ref() {
                                info!("Response chunk: {:?}", content);
                                string.push_str(content);
                            }
                        }),
                    Err(e) => error!("OpenAI Error: {:?}", e)
                }
            }
        }
    });

    rsx! {
        div {
            p { "Using OpenAI" }
            p { "User: {GREETING}" }
            p { "Assistant: {response_string}" }
        }
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}