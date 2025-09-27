use nn_yandex_art::Art;
use nn_yandex_art::models::request::message::MessageBuilder;
use nn_yandex_art::models::request::aspect_ratio::AspectRatioBuilder;
use nn_yandex_art::models::request::generation_options::GenerationOptionsBuilder;
use nn_yandex_art::models::request::types::ImageType;
use anyhow;

async fn generate_image(prompt: &str, path: &str) -> Result<(), anyhow::Error>{
    let message = MessageBuilder::new()
        .text(prompt)
        .weight(1)
        .build()?;

    let messages = vec![message];


    let aspect_ratio = AspectRatioBuilder::new()
        .width_ratio(1)
        .height_ratio(1)
        .build();

    let generation_options = GenerationOptionsBuilder::new()
        .aspect_ratio(aspect_ratio)
        .mime_type(ImageType::Png)
        .seed(121212121212) // !Optional
        .build()?;

    Ok(())
}