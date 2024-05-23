use tokio;
use clap::{Parser, Arg};
use sqlx::postgres::PgPoolOptions;
pub use sqlx::PgPool;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct LoginArgs {
    #[arg(short, long)]
    username: String
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let db_pool = db::setup_db_pool().await?;

    // let args = LoginArgs::parse();
    // println!("Welcome {}!", args.username);
    

    display_menu(&db_pool).await?;

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



async fn display_menu(db_pool: &PgPool) -> color_eyre::Result<()> {
    println!("Welcome to Knowledge!");
    println!("--------");

    loop {
        println!("1. Login");
        println!("2. Sign up");
        println!("3. Exit");

        println!("Enter your choice: ");
        let mut choice_str = String::new();
        std::io::stdin().read_line(&mut choice_str).expect("Failed to read line");
        let choice: i32 = match choice_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let username = login(&db_pool).await?;
                println!("Logged in as: {}", username);
            },
            2 => {
                sign_up(&db_pool).await?;
            },
            3 => break,
            _ => println!("Invalid choice. Please choose 1, 2, or 3."),
        }
    }

    Ok(())
}



async fn login(db_pool: &PgPool) -> color_eyre::Result<String> {
    println!("Enter a username ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim().to_string();
    // Add user to the database or verify existing user
    db::add_user(db_pool, &username).await?;
    println!("Welcome {}!", username);
    Ok(username)
}

async fn sign_up(db_pool: &PgPool) -> color_eyre::Result<()> {
    println!("Enter a username");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim().to_string();
    // Add new user to the database
    db::add_user(db_pool, &username).await?;
    println!("Signed up as: {}", username);
    Ok(())
}

pub async fn add_user(pool: &PgPool, username: &str) -> color_eyre::Result<()> {
    // Check if the user already exists
    let user_exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM Users WHERE user_name = $1)",
        username
    )
    .fetch_one(pool)
    .await?
    .exists
    .unwrap_or(false);

    if user_exists {
        println!("User {} already exists.", username);
        return Err(color_eyre::eyre::eyre!("User already exists"));
    }

    // Insert the new user
    sqlx::query!(
        "INSERT INTO Users (user_name) VALUES ($1)",
        username
    )
    .execute(pool)
    .await?;

    println!("User {} added successfully.", username);
    Ok(())
}
