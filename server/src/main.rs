#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().expect("`.env` file to be present");
    server::serve(([127, 0, 0, 1], 8080)).await;
}
