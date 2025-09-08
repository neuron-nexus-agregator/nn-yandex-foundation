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
cargo add nn_yandex_foundation
```

Dependencies included in the workspace:

* `yandex_art` – image generation
* `text_generation` – text generation
* `core` – shared configuration
* `test_examples` – examples

## Usage

### Text Generation

```rust
use text_generation::{TextGenerator, ModelType, Version};
use text_generation::models::request::{Request as TextRequest, CompletionOptions};
use text_generation::models::message::{Message, Role};

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
use nn_yandex_foundation::yandex_art

#[tokio::main]
async fn main() {
    const BUCKET: &str = "your-bucket-id";
    const API_KEY: &str = "your-api-key";

    let generator = yandex_art::Art::new(API.to_string(), BUCKET.to_string());

    let messages = vec![yandex_art::models::request::Message {
        text: "Dog".to_string(),
        weight: "1".to_string(),
    }];

    let options = yandex_art::models::request::GenerationOptions {
        mime_type: "image/png".to_string(),
        seed: None,
        aspect_ratio: yandex_art::models::request::AspectRatio {
            width_ratio: 1,
            height_ratio: 1,
        },
    };

    let req = yandex_art::models::request::Request::new(messages, options);

    let mut yandex_res = match generator.generate_image(req).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error generating image: {e}");
            return;
        }
    };

    let id = yandex_res.id.clone();

    while !yandex_res.done {
        sleep(Duration::from_secs(1)).await;
        yandex_res = match generator.check_operation(&id).await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error checking operation: {e}");
                continue;
            }
        };
    }

    if let Some(resp) = yandex_res.response {
        match STANDARD.decode(&resp.image) {
            Ok(bytes) => {
                match File::create("Image.png") {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(&bytes) {
                            eprintln!("Error writing file: {e}");
                        } else {
                            println!("Image successfully saved as Image.png");
                        }
                    }
                    Err(e) => eprintln!("Error creating file: {e}"),
                }
            }
            Err(e) => eprintln!("Error decoding Base64: {e}"),
        }
    } else {
        eprintln!("Response is missing image data");
    }
}
```

## Error Handling

* `GeneratorError` for text generation: HTTP, API, Unknown.
* `ArtError` for image generation: HTTP, API, NotReady, MissingResponse.

## Contributing

Contributions are welcome! Please open issues or pull requests on [GitHub](https://github.com/neuron-nexus-agregator/nn_yandex_foundation).

## License

This project is licensed under the MIT License.
