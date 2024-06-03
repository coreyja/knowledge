use color_eyre::Result;
use std::fs;
use std::io::Read;
use uuid::Uuid;

pub fn get_user_id_from_session() -> Result<Uuid> {
    let mut file = fs::File::open("auth.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let user_id = Uuid::parse_str(contents.trim())?;
    Ok(user_id)
}
