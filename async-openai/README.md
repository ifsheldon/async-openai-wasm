<h1 align="center"> async-openai-wasm </h1>
<p align="center"> Async Rust library for OpenAI on WASM</p>
<div align="center">
    <a href="https://crates.io/crates/async-openai-wasm">
    <img src="https://img.shields.io/crates/v/async-openai-wasm.svg" />
    </a>
    <a href="https://docs.rs/async-openai-wasm">
    <img src="https://docs.rs/async-openai-wasm/badge.svg" />
    </a>
</div>

## Overview

`async-openai-wasm`**is a FORK of `async-openai`** that supports WASM targets by targeting `wasm32-unknown-unknown`.
That means >99% of the codebase should be attributed to the original project. The synchronization with the original
project is and will be done manually when `async-openai` releases a new version. Versions are kept in sync
with `async-openai` releases, which means when `async-openai` releases `x.y.z`, `async-openai-wasm` also releases
a `x.y.z` version.

`async-openai-wasm` is an unofficial Rust library for OpenAI, based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi). It implements all APIs from the spec:

| What | APIs | Crate Feature Flags |
|---|---|---|
| **Responses API** | Responses, Conversations, Streaming events | `responses` |
| **Webhooks** | Webhook Events | `webhook` |
| **Platform APIs** | Audio, Audio Streaming, Videos, Images, Image Streaming, Embeddings, Evals, Fine-tuning, Graders, Batch, Files, Uploads, Models, Moderations | `audio`, `video`, `image`, `embedding`, `evals`, `finetuning`, `grader`, `batch`, `file`, `upload`, `model`, `moderation` |
| **Vector stores** | Vector stores, Vector store files, Vector store file batches | `vectorstore` |
| **ChatKit** <sub>(Beta)</sub> | ChatKit | `chatkit` |
| **Containers** | Containers, Container Files | `container` |
| **Realtime** | Realtime Calls, Client secrets, Client events, Server events | `realtime` |
| **Chat Completions** | Chat Completions, Streaming | `chat-completion` |
| **Assistants** <sub>(Beta)</sub> | Assistants, Threads, Messages, Runs, Run steps, Streaming | `assistant` |
| **Administration** | Admin API Keys, Invites, Users, Groups, Roles, Role assignments, Projects, Project users, Project groups, Project service accounts, Project API keys, Project rate limits, Audit logs, Usage, Certificates | `administration` |
| **Legacy** | Completions | `completions` |

Features that makes `async-openai` unique:
- Bring your own custom types for Request or Response objects.
- SSE streaming on available APIs.
- Customize path, query and headers per request; customize path and headers globally (for all requests).
- Requests (except SSE streaming) including form submissions are retried with exponential backoff when [rate limited](https://platform.openai.com/docs/guides/rate-limits).
- Ergonomic builder pattern for all request objects.
- Granular feature flags to enable any types or apis: good for faster compilation and crate reuse.
- Microsoft Azure OpenAI Service (only for APIs matching OpenAI spec).

More on `async-openai-wasm`:
- **WASM support**
- Reasoning Model Support: support models like DeepSeek R1 via broader support for OpenAI-compatible endpoints, see `examples/reasoning`

**Note on Azure OpenAI Service (AOS)**:  `async-openai-wasm` primarily implements OpenAI spec, and doesn't try to
maintain parity with spec of AOS. Just like `async-openai`.

## Differences from `async-openai`

```diff
+ * WASM support
+ * WASM examples
+ * Realtime API: Does not bundle with a specific WS implementation. Need to convert a client event into a WS message by yourself, which is just simple `your_ws_impl::Message::Text(some_client_event.into_text())`
+ * Broader support for OpenAI-compatible Endpoints
+ * Reasoning Model Support
- * Tokio
- * Non-wasm examples: please refer to the original project [async-openai](https://github.com/64bit/async-openai/).
- * Builtin backoff retries: due to [this issue](https://github.com/ihrwein/backoff/issues/61). 
-   * Recommend: use `backon` with `gloo-timers-sleep` feature instead.
- * File saving: `wasm32-unknown-unknown` on browsers doesn't have access to filesystem.
```

## Usage

The library reads [API key](https://platform.openai.com/account/api-keys) from the environment
variable `OPENAI_API_KEY`.

```bash
# On macOS/Linux
export OPENAI_API_KEY='sk-...'
```

```powershell
# On Windows Powershell
$Env:OPENAI_API_KEY='sk-...'
```

Other official environment variables supported are: `OPENAI_ADMIN_KEY`, `OPENAI_BASE_URL`, `OPENAI_ORG_ID`, `OPENAI_PROJECT_ID`

- Visit [examples](https://github.com/64bit/async-openai/tree/main/examples) directory on how to use `async-openai`,
  and [WASM examples](https://github.com/ifsheldon/async-openai-wasm/tree/main/examples) in `async-openai-wasm`.
- Visit [docs.rs/async-openai](https://docs.rs/async-openai) for docs.


## Image Generation Example

```rust
use async_openai_wasm::{
    types::images::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .prompt("cats on sofa and carpet in living room")
        .n(2)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai-wasm")
        .build()?;

    let response = client.images().generate(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
```

<div align="center">
  <img width="315" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image/img-1.png" />
  <img width="315" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image/img-2.png" />
  <br/>
  <sub>Scaled up for README, actual size 256x256</sub>
</div>

## Webhooks

Support for webhook includes event types, signature verification, and building webhook events from payloads.

## Bring Your Own Types

Enable methods whose input and outputs are generics with `byot` feature. It creates a new method with same name and `_byot` suffix. 

`byot` requires trait bounds: 
- a request type (`fn` input parameter) needs to implement `serde::Serialize` or `std::fmt::Display` trait
- a response type (`fn` ouput parameter) needs to implement `serde::de::DeserializeOwned` trait.

For example, to use `serde_json::Value` as request and response type:
```rust
let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "developer",
                    "content": "You are a helpful assistant"
                },
                {
                    "role": "user",
                    "content": "What do you think about life?"
                }
            ],
            "model": "gpt-4o",
            "store": false
        }))
        .await?;
```

This can be useful in many scenarios:
- To use this library with other OpenAI compatible APIs whose types don't exactly match OpenAI. 
- Extend existing types in this crate with new fields with `serde` (for example with `#[serde(flatten)]`).
- To avoid verbose types.
- To escape deserialization errors.

Visit [examples/bring-your-own-type](https://github.com/64bit/async-openai/tree/main/examples/bring-your-own-type)
directory to learn more.

### References: Borrow Instead of Move

With `byot` use reference to request types

```rust
let response: Response = client
  .responses()
  .create_byot(&request).await?
```

Visit [examples/borrow-instead-of-move](https://github.com/64bit/async-openai/tree/main/examples/borrow-instead-of-move) to learn more.


## Rust Types

To only use Rust types from the crate - disable default features and use feature flag `types`. 

There are granular feature flags like `response-types`, `chat-completion-types`, etc.

These granular types are enabled when the corresponding API feature is enabled - for example `response` will enable `response-types`.

## Configurable Requests

### Individual Request
Certain individual APIs that need additional query or header parameters - these can be provided by chaining `.query()`, `.header()`, `.headers()` on the API group. 

For example:
```rust
client.
  .chat()
  // query can be a struct or a map too.
  .query(&[("limit", "10")])?
  // header for demo
  .header("key", "value")?
  .list()
  .await?
```

### All Requests

Use `Config`, `OpenAIConfig` etc. for configuring url, headers or query parameters globally for all requests.

## OpenAI-compatible Providers

Even though the scope of the crate is official OpenAI APIs, it is very configurable to work with compatible providers.

### Configurable Path

In addition to  `.query()`, `.header()`, `.headers()` a path for individual request can be changed by using `.path()`,  method on the API group.

For example:

```rust
client
  .chat()
  .path("/v1/messages")?
  .create(request)
  .await?
```

### Dynamic Dispatch

This allows you to use same code (say a `fn`) to call APIs on different OpenAI-compatible providers.

For any struct that implements `Config` trait, wrap it in a smart pointer and cast the pointer to `dyn Config`
trait object, then create a client with `Box` or `Arc` wrapped configuration.

For example:

```rust
use async_openai::{Client, config::{Config, OpenAIConfig}};

// Use `Box` or `std::sync::Arc` to wrap the config
let config = Box::new(OpenAIConfig::default()) as Box<dyn Config>;
// create client
let client: Client<Box<dyn Config>> = Client::with_config(config);

// A function can now accept a `&Client<Box<dyn Config>>` parameter
// which can invoke any openai compatible api
fn chat_completion(client: &Client<Box<dyn Config>>) { 
    todo!() 
}
```

## Contributing

This repo will only accept issues and PRs related to WASM support. For other issues and PRs, please visit the original
project [async-openai](https://github.com/64bit/async-openai).


## Why `async-openai-wasm`

Because I wanted to develop and release a crate that depends on the wasm feature in `experiments` branch
of [async-openai](https://github.com/64bit/async-openai), but the pace of stabilizing the wasm feature is different
from what I expected.
- [openai-func-enums](https://github.com/frankfralick/openai-func-enums) macros for working with function/tool calls.

## License

The additional modifications are licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
The original project is also licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
