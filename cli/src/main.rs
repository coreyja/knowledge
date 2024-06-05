use clap::{Parser, Subcommand};

mod add_url;
use add_url::append_url;

mod auth;
use auth::check_auth_status;
use auth::get_user_id_from_session;

mod sign_up;
// use db::urls::process_page_snapshot;
// use db::urls::AddUrlOutcome;
use sign_up::sign_up;

mod display_user;
use display_user::display_users;

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
            append_url(&db_pool, &url, user_id, allow_existing).await?;
            // let outcome = append_url(&db_pool, &url, user_id, allow_existing).await?;
            // if let AddUrlOutcome::Created(page) | AddUrlOutcome::Existing(page) = outcome {
            //     process_page_snapshot(&db_pool, page.page_id, user_id, &url).await?;
            // }
        }
    }

    Ok(())
}
