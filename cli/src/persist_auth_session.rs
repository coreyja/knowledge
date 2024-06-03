use std::fs;
use std::io::Write;
use uuid::Uuid;

pub fn persist_auth_session(user_id: Uuid) -> color_eyre::Result<()> {
    let mut file = fs::File::create("auth.txt")?;
    writeln!(file, "{user_id}")?;

    Ok(())
}
