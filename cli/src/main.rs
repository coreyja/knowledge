#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let db_pool = db::setup_db_pool().await?;

    println!("Hello, world!");

    Ok(())
}
