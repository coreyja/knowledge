use db::PgPool;
use std::fs;
use std::io::Read;
use std::path::Path;
use uuid::Uuid;

pub async fn check_auth_status(pool: &PgPool) -> color_eyre::Result<()> {
    let user_path = Path::new("auth.txt");
    if user_path.exists() {
        let mut file = fs::File::open("auth.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let user_id = Uuid::parse_str(contents.trim())?;

        match db::get_username_by_id(pool, user_id).await? {
            Some(username) => println!("Logged in with Username: {username}, User ID: {user_id}"),
        None => return Err(color_eyre::eyre::eyre!("User ID not found in database.")),
        }
    } else {
        return Err(color_eyre::eyre::eyre!("Not logged in."));
    }
    Ok(())

}
