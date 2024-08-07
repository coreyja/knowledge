use clap::{Parser, Subcommand};

mod add_url;
use add_url::append_url;

mod auth;
use auth::check_auth_status;
use auth::get_user_id_from_session;

mod sign_up;
use cores::page_snapshot::clean_raw_html;
use cores::page_snapshot::download_raw_html;
use cores::urls::AddUrlOutcome;
use sign_up::sign_up;

use url::Url;

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
    let db_pool = cores::setup_db_pool().await?;

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
            let raw_html = download_raw_html(&url).await?;
            let url_parsed = Url::parse(&url)?;
            let _cleaned_html = clean_raw_html(&raw_html, &url_parsed)?;
            let outcome = append_url(&db_pool, &url, user_id, allow_existing).await?;
            let _page = match outcome {
                AddUrlOutcome::Created(page) | AddUrlOutcome::Existing(page) => page,
            };
        }
    }

    Ok(())
}
