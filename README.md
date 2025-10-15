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
use nn_yandex_gpt::models::request::{RequestBuilder, CompletionOptions};
use nn_yandex_gpt::models::message::{MessageBuilder, Role};

pub async fn generate_text(text: &str, prompt: &str) -> Result<String, anyhow::Error> {

    let BUCKET = "bucket".to_string();
    let API_KEY = "api".to_string();

    let prompt_message = MessageBuilder::new()
        .with_role(Role::System)
        .with_text(prompt)
        .build();

    let user_message = MessageBuilder::new()
        .with_role(Role::User)
        .with_text(text)
        .build();

    let opts = CompletionOptions::new()
        .with_temperature(0.3)
        .with_max_tokens(1024);

    let req = RequestBuilder::new()
        .message(prompt_message)
        .message(user_message)
        .with_completion_options(opts)
        .build();

    let generator = TextGenerator::new(API_KEY, BUCKET);
    let result = generator.complete(ModelType::GptPro, Version::RC, req).await;
    match result {
        Ok(result) => {
            let alt = result.result.alternatives[0].clone();
            Ok(alt.message.text)
        },
        Err(err) => Err(anyhow::Error::new(err)),
    }
}

pub async fn start_chating(){
    use std::io::{stdin,stdout,Write};

    let BUCKET = "bucket".to_string();
    let API_KEY = "api".to_string();

    let prompt_message = MessageBuilder::new()
        .with_role(Role::System)
        .with_text("Ты — профессиональный ассистент")
        .build();

    let assistant_message = MessageBuilder::new()
        .with_role(Role::System)
        .with_text("Чем я могу вам помочь?")
        .build();

    let opts = CompletionOptions::new()
        .with_temperature(0.7);

    let mut req = RequestBuilder::new()
        .message(prompt_message)
        .message(assistant_message)
        .with_completion_options(opts)
        .build();

    let generator = TextGenerator::new(API_KEY, BUCKET);

    println!("Ассистент: Чем я могу вам помочь?", );
    loop {
        print!("Вы: ");
        let mut s=String::new();
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let user_message = MessageBuilder::new()
            .with_role(Role::User)
            .with_text(&s)
            .build();
        req.messages.push(user_message);
        let result = generator.complete(ModelType::GptPro, Version::RC, req.clone()).await;
        match result {
            Ok(result) => {
                let alt = result.result.alternatives[0].clone();
                req.messages.push(alt.message.clone());
                println!("Ассистент: {}", alt.message.text);
            },
            Err(err) => println!("{err}"),
        }
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
