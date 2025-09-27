mod image_generation;
use image_generation::generate_image;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let prompt = "Кошка сидит на диване";
    let path = "./images/image4.png";
    let width_ratio: i64 = 3;
    let height_ratio: i64 = 2;
    match generate_image(prompt, path, width_ratio, height_ratio).await{
        Ok(_) => println!("Image generated successfully"),
        Err(e) => eprintln!("{e}"),
    }
}

