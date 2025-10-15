mod image_generation;
mod text_generation;

#[tokio::main]
async fn main() {
    text_generation::start_chating().await;
}

