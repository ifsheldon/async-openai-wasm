# OpenAI Web App - Assistant

This builds a `dioxus` web App that uses OpenAI Assistant APIs to generate text.

To run it, you need:
1. Set OpenAI secrets in `./src/main.rs`. Please do NOT take this demo into production without using a secure secret store
2. Install `dioxus-cli` by `cargo install dioxus-cli`.
3. Run `dx serve`

Note: Safari may not work due to CORS issues. Please use Chrome or Edge.

## Reference

The code is adapted from [assistant-func-call-stream example in async-openai](https://github.com/64bit/async-openai/tree/main/examples/assistants-func-call-stream).