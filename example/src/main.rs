mod image_generation;
use image_generation::generate_image;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let prompt = "Dog in sofa";
    let path = "./image.png";
    match generate_image(prompt, path).await{
        Ok(_) => println!("Image generated successfully"),
        Err(e) => eprintln!("{e}"),
    }
}

