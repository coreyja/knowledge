#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let _db_pool = db::setup_db_pool().await?;

    display_menu();

    Ok(())
}

fn display_menu() -> color_eyre::Result<()> {

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
                let username = login();
                println!("Logged in as: {}", username);
            },
            2 => sign_up(),
            3 => break,
            _ => println!("Invalid choice. Please choose 1, 2, or 3."),
        }
    }

    Ok(())
}

fn login() -> String {
    println!("Enter a username ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim().to_string();
    println!("Welcome {}!", username);
    username
}

fn sign_up() {
    println!("Enter a username");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim().to_string();
    println!("Signed up as: {}", username);
}