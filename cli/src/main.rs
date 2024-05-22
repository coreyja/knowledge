#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let _db_pool = db::setup_db_pool().await?;

    println!("Hello, world!");

    Ok(())
}
