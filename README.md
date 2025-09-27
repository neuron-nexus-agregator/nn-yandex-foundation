# nn\_yandex\_foundation

Unified library for working with **Yandex Foundation Models**. Provides a simple interface for generating text and images using Yandex's AI services.

Repository: [https://github.com/neuron-nexus-agregator/nn\_yandex\_foundation](https://github.com/neuron-nexus-agregator/nn_yandex_foundation)

---

## Features

* Generate text using Yandex GPT models (`GptLite`, `GptPro`, `Llama8B`, `Llama70B`).
* Choose model version (`Latest`, `RC`, `Deprecated`).
* Generate images using Yandex Art models asynchronously.
* Check the status of ongoing image generation operations.
* Unified error handling with clear error types for HTTP, API, and unknown issues.
* Fully asynchronous and compatible with Rust async runtimes.

## Installation

Add the workspace crate to your `Cargo.toml`:

```curl
cargo add nn_yandex_gpt
cargo add nn_yandex_art
```

Dependencies included in the workspace:

* `yandex_art` – image generation
* `text_generation` – text generation

## Usage

### Text Generation

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

### Image Generation

```rust
use nn_yandex_art::Art;
use nn_yandex_art::models::request::message::MessageBuilder;
use nn_yandex_art::models::request::aspect_ratio::AspectRatioBuilder;
use nn_yandex_art::models::request::generation_options::GenerationOptionsBuilder;
use nn_yandex_art::models::request::types::ImageType;
use nn_yandex_art::models::request::RequestBuilder;
use anyhow;
use tokio::time::{sleep, Duration};
use std::fs::File;
use std::io::Write;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::env;

pub async fn generate_image(prompt: &str, path: &str) -> Result<(), anyhow::Error>{

    let BUCKET = env::var("BUCKET")?;
    let API_KEY = env::var("API")?;

    let message = MessageBuilder::new()
        .text(prompt)
        .weight(1)
        .build()?;


    let aspect_ratio = AspectRatioBuilder::new()
        .width_ratio(1)
        .height_ratio(1)
        .build();

    let generation_options = GenerationOptionsBuilder::new()
        .aspect_ratio(aspect_ratio)
        .mime_type(ImageType::Png)
        //.seed(121212121212) // !Optional
        .build()?;

    let request = RequestBuilder::new()
        .generation_options(generation_options)
        .message(message)
        .build()?;

    let art = Art::new(API_KEY, BUCKET);
    let mut res = art.generate_image(request).await?;
    let id = res.id;

    if let Some(e) = res.error{
        return Err(anyhow::anyhow!("{}", e.message))
    }

    while !res.done{
        sleep(Duration::from_secs(1)).await;
        res = art.check_operation(&id).await?
    }

    if let Some(resp) = res.response {
        save_image(resp.image, path)
    } else {
        Err(anyhow::anyhow!("Response is missing image data"))
    }
}

fn save_image(image: String, path: &str) -> Result<(), anyhow::Error>{
    match STANDARD.decode(image) {
        Ok(bytes) => {
            match File::create(path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&bytes) {
                        Err(anyhow::anyhow!("Error writing file: {e}"))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        }
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}
```

## Error Handling

* `ArtError` for image generation: HTTP, API, NotReady, MissingResponse.
* `BuildError` for struct builders

## Contributing

Contributions are welcome! Please open issues or pull requests on [GitHub](https://github.com/neuron-nexus-agregator/nn-yandex-foundation).

## License

This project is licensed under the MIT License.
