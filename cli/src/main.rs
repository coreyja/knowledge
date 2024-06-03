use clap::{Parser, Subcommand};
use db::PgPool;

mod add_url;
use add_url::add_url;

mod auth;
use auth::check_auth_status;
use auth::get_user_id_from_session;

mod sign_up;
use sign_up::sign_up;

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
    AddUrl {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        allow_existing: bool,
    },
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
            check_auth_status(&db_pool).await?;
        }
        Command::AddUrl {
            url,
            allow_existing,
        } => {
            let user_id = get_user_id_from_session()?;
            add_url(&db_pool, &url, user_id, allow_existing).await?;
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
