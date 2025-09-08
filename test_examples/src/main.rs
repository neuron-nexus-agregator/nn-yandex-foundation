use std::fs::File;
use std::io::Write;
use std::time::Duration;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use tokio::time::sleep;
use yandex_art;
use text_generation::{TextGenerator, ModelType, Version};
use text_generation::models::request::{Request as TextRequest, CompletionOptions};
use text_generation::models::message::{Message, Role};

const BUCKET: &str = "123456777845645";
const API: &str = "KdksfdsfdsaklJkjksdfjk12312jkdsjkdsfjkds";



#[tokio::main]
async fn main() {
    test_text_generation().await;
    test_image_generation().await;
}

async fn test_text_generation() {
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

async fn test_image_generation() {
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
