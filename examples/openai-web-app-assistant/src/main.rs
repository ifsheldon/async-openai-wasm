#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info, Level};
use futures::stream::StreamExt;

use async_openai_wasm::types::{AssistantStreamEvent, CreateMessageRequest, CreateRunRequest, CreateThreadRequest, MessageRole};

use crate::utils::*;

mod utils;

pub const API_BASE: &str = "...";
pub const API_KEY: &str = "...";


pub fn App() -> Element {
    const QUERY: &str = "What's the weather in San Francisco today and the likelihood it'll rain?";
    let reply = use_signal(String::new);
    let _run_assistant: Coroutine<()> = use_coroutine(|_rx| {
        let client = get_client();
        async move {
            //
            // Step 1: Define functions
            //
            let assistant = client
                .assistants()
                .create(create_assistant_request())
                .await
                .expect("failed to create assistant");
            //
            // Step 2: Create a Thread and add Messages
            //
            let thread = client
                .threads()
                .create(CreateThreadRequest::default())
                .await
                .expect("failed to create thread");
            let _message = client
                .threads()
                .messages(&thread.id)
                .create(CreateMessageRequest {
                    role: MessageRole::User,
                    content: QUERY.into(),
                    ..Default::default()
                })
                .await
                .expect("failed to create message");
            //
            // Step 3: Initiate a Run
            //
            let mut event_stream = client
                .threads()
                .runs(&thread.id)
                .create_stream(CreateRunRequest {
                    assistant_id: assistant.id.clone(),
                    stream: Some(true),
                    ..Default::default()
                })
                .await
                .expect("failed to create run");


            while let Some(event) = event_stream.next().await {
                match event {
                    Ok(event) => match event {
                        AssistantStreamEvent::ThreadRunRequiresAction(run_object) => {
                            info!("thread.run.requires_action: run_id:{}", run_object.id);
                            handle_requires_action(&client, run_object, reply.to_owned()).await
                        }
                        _ => info!("\nEvent: {event:?}\n"),
                    },
                    Err(e) => {
                        error!("Error: {e}");
                    }
                }
            }

            client.threads().delete(&thread.id).await.expect("failed to delete thread");
            client.assistants().delete(&assistant.id).await.expect("failed to delete assistant");
            info!("Done!");
        }
    });

    rsx! {
        div {
            p { "Using OpenAI" }
            p { "User: {QUERY}" }
            p { "Expected Stats (Debug): temperature = {TEMPERATURE}, rain_probability = {RAIN_PROBABILITY}" }
            p { "Assistant: {reply}" }
        }
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}