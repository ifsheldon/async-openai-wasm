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

`async-openai-wasm` is an unofficial Rust library for OpenAI.

- It's based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi)
- Current features:
    - [x] Assistants (v2)
    - [x] Audio
    - [x] Batch
    - [x] Chat
    - [x] Completions (Legacy)
    - [x] Embeddings
    - [x] Files
    - [x] Fine-Tuning
    - [x] Images
    - [x] Models
    - [x] Moderations
    - [ ] Organizations | Administration
    - [x] Realtime API types (Beta)
    - [ ] Uploads
    - [x] **WASM support**
- SSE streaming on available APIs
- Ergonomic builder pattern for all request objects.
- Microsoft Azure OpenAI Service (only for APIs matching OpenAI spec)

**Note on Azure OpenAI Service (AOS)**:  `async-openai-wasm` primarily implements OpenAI spec, and doesn't try to
maintain parity with spec of AOS. Just like `async-openai`.

## Differences from `async-openai`

```diff
+ * WASM support
+ * WASM examples
+ * Realtime API: Does not bundle with a specific WS implementation. Need to convert a client event into a WS message by yourself, which is just simple `your_ws_impl::Message::Text(some_client_event.into_text())`
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

- Visit [examples](https://github.com/64bit/async-openai/tree/main/examples) directory on how to use `async-openai`,
  and [WASM examples](https://github.com/ifsheldon/async-openai-wasm/tree/main/examples) in `async-openai-wasm`.
- Visit [docs.rs/async-openai](https://docs.rs/async-openai) for docs.

## Realtime API

Only types for Realtime API are implemented, and can be enabled with feature flag `realtime`
These types may change when OpenAI releases official specs for them.

Again, the types do not bundle with a specific WS implementation. Need to convert a client event into a WS message by yourself, which is just simple `your_ws_impl::Message::Text(some_client_event.into_text())`.

## Image Generation Example

```rust
use async_openai_wasm::{
    types::{CreateImageRequestArgs, ImageSize, ImageResponseFormat},
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

    let response = client.images().create(request).await?;

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

## Contributing

This repo will only accept issues and PRs related to WASM support. For other issues and PRs, please visit the original
project [async-openai](https://github.com/64bit/async-openai).

This project adheres to [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## Complimentary Crates

- [openai-func-enums](https://github.com/frankfralick/openai-func-enums) provides procedural macros that make it easier
  to use this library with OpenAI API's tool calling feature. It also provides derive macros you can add to
  existing [clap](https://github.com/clap-rs/clap) application subcommands for natural language use of command line
  tools. It also supports
  openai's [parallel tool calls](https://platform.openai.com/docs/guides/function-calling/parallel-function-calling) and
  allows you to choose between running multiple tool calls concurrently or own their own OS threads.

## Why `async-openai-wasm`

Because I wanted to develop and release a crate that depends on the wasm feature in `experiments` branch
of [async-openai](https://github.com/64bit/async-openai), but the pace of stabilizing the wasm feature is different
from what I expected.

## License

The additional modifications are licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
The original project is also licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
