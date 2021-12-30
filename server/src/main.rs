#[rocket::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    server::build_rocket().await?.launch().await?;

    Ok(())
}
