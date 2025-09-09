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
nn_yandex_art = "0.1.0"
```

## Example

```rust
use nn_yandex_art
use std::fs::File;
use std::io::Write;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use tokio::time::{sleep, Duration};

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

* `ArtError` for image generation: HTTP, API, NotReady, MissingResponse.

## Contributing

Contributions are welcome! Please open issues or pull requests on [GitHub](https://github.com/neuron-nexus-agregator/nn-yandex-foundation).

## License

This project is licensed under the MIT License.
