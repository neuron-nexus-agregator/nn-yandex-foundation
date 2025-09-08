# nn\_yandex\_gpt

Rust client for **Yandex GPT API**.
This crate allows developers to easily generate text completions using Yandex Foundation Models.

## Features

* Supports multiple models: GptLite, GptPro, Llama8B, Llama70B
* Select version: Deprecated, Latest, RC
* Handles HTTP and API errors with structured results

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nn_yandex_gpt = "0.1.0"
```

## Example

```rust
use nn_yandex_gpt::{TextGenerator, ModelType, Version};
use nn_yandex_gpt::models::request::{Request as TextRequest, CompletionOptions};
use nn_yandex_gpt::models::message::{Message, Role};

#[tokio::main]
async fn main() {
    const BUCKET: &str = "your-bucket-id";
    const API_KEY: &str = "your-api-key";

    let generator = TextGenerator::new(API.to_string(), BUCKET.to_string());

    let messages = vec![
        Message {
            role: Role::System,
            text: Some("You are a professional mathematician".to_string()),
            tool_call_list: None,
            tool_result_list: None,
        },
        Message {
            role: Role::User,
            text: Some("Write a fictional proof that 2 + 2 = 5".to_string()),
            tool_call_list: None,
            tool_result_list: None,
        }
    ];

    let mut req = TextRequest::new(messages);

    let mut completion_options = CompletionOptions::new();
    completion_options.temperature = Some(0.3);
    completion_options.max_tokens = Some(500);
    completion_options.reasoning_options = None;

    req.completion_options = Some(completion_options);

    match generator.complete(ModelType::GptPro, Version::RC, req).await {
        Ok(res) => println!("Text generation result: {:?}", res.result.alternatives[0].message.text),
        Err(e) => eprintln!("Error during text generation: {}", e),
    }
}
```

## License

MIT
