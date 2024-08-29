use std::error::Error;
use dioxus::prelude::Signal;
use futures::StreamExt;
use tracing::{error, info};
use async_openai_wasm::Client;
use async_openai_wasm::config::OpenAIConfig;
use async_openai_wasm::types::{AssistantStreamEvent, CreateAssistantRequest, CreateAssistantRequestArgs, FunctionObject, MessageDeltaContent, RunObject, SubmitToolOutputsRunRequest, ToolsOutputs};
use crate::{API_BASE, API_KEY};
use dioxus::prelude::*;


pub const TEMPERATURE: &str = "57";
pub const RAIN_PROBABILITY: &str = "0.06";

pub fn get_client() -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new()
        .with_api_key(API_KEY);
    let config = if API_BASE != "..." {
        config.with_api_base(API_BASE)
    } else {
        config
    };

    Client::with_config(config)
}

pub async fn handle_requires_action(client: &Client<OpenAIConfig>, run_object: RunObject, reply_signal: Signal<String>) {
    let mut tool_outputs: Vec<ToolsOutputs> = vec![];
    if let Some(ref required_action) = run_object.required_action {
        for tool in &required_action.submit_tool_outputs.tool_calls {
            if tool.function.name == "get_current_temperature" {
                info!("get_current_temperature");
                tool_outputs.push(ToolsOutputs {
                    tool_call_id: Some(tool.id.clone()),
                    output: Some(TEMPERATURE.into()),
                })
            } else if tool.function.name == "get_rain_probability" {
                info!("get_rain_probability");
                tool_outputs.push(ToolsOutputs {
                    tool_call_id: Some(tool.id.clone()),
                    output: Some(RAIN_PROBABILITY.into()),
                })
            } else {
                error!("Unknown tool: {}", tool.function.name);
                unreachable!();
            }
        }

        if let Err(e) = submit_tool_outputs(client, run_object, tool_outputs, reply_signal).await {
            error!("Error on submitting tool outputs: {e}");
        }
    }
}

pub async fn submit_tool_outputs(
    client: &Client<OpenAIConfig>,
    run_object: RunObject,
    tool_outputs: Vec<ToolsOutputs>,
    mut reply_signal: Signal<String>,
) -> Result<(), Box<dyn Error>> {
    let mut event_stream = client
        .threads()
        .runs(&run_object.thread_id)
        .submit_tool_outputs_stream(
            &run_object.id,
            SubmitToolOutputsRunRequest {
                tool_outputs,
                stream: Some(true),
            },
        )
        .await?;

    while let Some(event) = event_stream.next().await {
        match event {
            Ok(event) => {
                if let AssistantStreamEvent::ThreadMessageDelta(delta) = event {
                    if let Some(contents) = delta.delta.content {
                        for content in contents {
                            // only text is expected here and no images
                            if let MessageDeltaContent::Text(text) = content {
                                if let Some(text) = text.text {
                                    if let Some(text) = text.value {
                                        info!("After submitted tool results: {}", text);
                                        reply_signal.with_mut(|reply| {
                                            reply.push_str(&text);
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error: {e}");
            }
        }
    }

    Ok(())
}

pub fn create_assistant_request() -> CreateAssistantRequest {
    CreateAssistantRequestArgs::default()
        .instructions("You are a weather bot. Use the provided functions to answer questions.")
        .model("gpt-4o")
        .tools(vec![
            FunctionObject {
                name: "get_current_temperature".into(),
                description: Some("Get the current temperature for a specific location".into()),
                parameters: Some(serde_json::json!(
                {
                    "type": "object",
                    "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g., San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["Celsius", "Fahrenheit"],
                        "description": "The temperature unit to use. Infer this from the user's location."
                    }
                    },
                    "required": ["location", "unit"]
                }
            )),
                strict: None,
            }.into(),
            FunctionObject {
                name: "get_rain_probability".into(),
                description: Some("Get the probability of rain for a specific location".into()),
                parameters: Some(serde_json::json!(
                {
                    "type": "object",
                    "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g., San Francisco, CA"
                    }
                    },
                    "required": ["location"]
                }
            )),
                strict: None,
            }.into(),
        ])
        .build()
        .expect("failed to build CreateAssistantRequestArgs")
}