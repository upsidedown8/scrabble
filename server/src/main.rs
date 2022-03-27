/// The main entry point for the server.
#[tokio::main]
async fn main() -> server::error::Result<()> {
    env_logger::init();
    dotenv::dotenv().expect("`.env` file to be present");
    server::serve(([127, 0, 0, 1], 8000)).await?;

    Ok(())
}
