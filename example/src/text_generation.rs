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
