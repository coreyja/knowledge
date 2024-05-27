use db::PgPool;
use tokio;
use clap::{Parser, Subcommand};
use db::sqlx;
use std::fs;
use std::io::Write;
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
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = KnowledgeArgs::parse();
    let db_pool = db::setup_db_pool().await?;

    match args.command {
        Command::Login => {
            println!("Login not implemented yet")
        },
        Command::Signup { username } => {
            sign_up(&db_pool, username).await?;
        },
        Command::DisplayUsers => {
            display_users(&db_pool).await?;
        }
    }

    Ok(())
}

async fn persist_auth_session(user_id: Uuid) -> color_eyre::Result<()> {
    if fs::metadata("auth.txt").is_ok() {
        return Err(color_eyre::eyre::eyre!("auth.txt already exists"));
    }

    let mut file = fs::File::create("auth.txt")?;
    writeln!(file, "{}", user_id)?;

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
    let username = match username_opt {
        Some(name) => name,
        None => {
            println!("Enter a username:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string()
        }
    };

    add_user(pool, &username).await?;
    println!("Signed up as: {}", username);
    Ok(())
}

async fn add_user(pool: &PgPool, username: &str) -> color_eyre::Result<()> {
    let result = sqlx::query!(
        "INSERT INTO Users (user_name) VALUES ($1) RETURNING user_id",
        username
    )
    .fetch_one(pool)
    .await?;

    persist_auth_session(result.user_id).await?;

    println!("User {} added successfully with ID {}!", username, result.user_id);
    Ok(())
}
