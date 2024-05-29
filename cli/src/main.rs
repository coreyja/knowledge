use clap::{Parser, Subcommand};
use db::PgPool;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct KnowledgeArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Login,
    Signup {
        #[arg(short, long)]
        username: Option<String>,
    },
    DisplayUsers,
    Status,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = KnowledgeArgs::parse();
    let db_pool = db::setup_db_pool().await?;

    match args.command {
        Command::Login => {
            println!("Login not implemented yet");
        }
        Command::Signup { username } => {
            sign_up(&db_pool, username).await?;
        }
        Command::DisplayUsers => {
            display_users(&db_pool).await?;
        }
        Command::Status => {
            check_status(&db_pool).await?;
        }
    }

    Ok(())
}

async fn display_users(pool: &PgPool) -> color_eyre::Result<()> {
    let users = db::get_users(pool).await?;
    if users.is_empty() {
        println!("Nothing to display");
    } else {
        for user in users {
            println!("User ID: {}, Username: {}", user.user_id, user.user_name);
        }
    }

    Ok(())
}

async fn sign_up(pool: &PgPool, username_opt: Option<String>) -> color_eyre::Result<()> {
    if Path::new("auth.txt").exists() {
        check_status(pool).await?;
        return Ok(());
    }

    let username = if let Some(name) = username_opt {
        name
    } else {
        println!("Enter a username:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    };

    add_user(pool, &username).await?;
    println!("Signed up as: {username}");
    Ok(())
}

async fn check_status(pool: &PgPool) -> color_eyre::Result<()> {
    let user_path = Path::new("auth.txt");
    if user_path.exists() {
        let mut file = fs::File::open("auth.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let user_id = Uuid::parse_str(contents.trim())?;

        match db::get_username_by_id(pool, user_id).await? {
            Some(username) => println!("Logged in with Username: {username}, User ID: {user_id}"),
            None => println!("file empty"),
        }
    } else {
        println!("Not logged in.");
    }
    Ok(())
}

async fn add_user(pool: &PgPool, username: &str) -> color_eyre::Result<()> {
    let user_id = db::create_user(pool, username).await?;
    persist_auth_session(user_id)?;

    println!("User {username} added successfully with ID {user_id}!");
    Ok(())
}

fn persist_auth_session(user_id: Uuid) -> color_eyre::Result<()> {
    let mut file = fs::File::create("auth.txt")?;
    writeln!(file, "{user_id}")?;

    Ok(())
}
