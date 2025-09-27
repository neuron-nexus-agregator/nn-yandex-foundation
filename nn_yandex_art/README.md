# nn\_yandex\_art

Unified Rust client for working with **Yandex Art API**.
This crate provides a convenient way to generate and fetch images from Yandex Foundation Models.

## Features

* Generate images with text prompts
* Check operation status by operation ID
* Decode Base64 images into files

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nn_yandex_art = "0.2.0"
```

## Example

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
