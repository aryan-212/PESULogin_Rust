use clap::Parser;
use dotenv::dotenv;
use std::env;
use std::process::Command;

mod build;
mod connection_initialisation;

#[derive(Parser, Debug)]
#[command(name = "pesl", about = "Wi-Fi Login/Logout CLI")]
struct Args {
    /// Login to Wi-Fi
    #[arg(long)]
    login: bool,
    
    /// Logout from Wi-Fi
    #[arg(long)]
    logout: bool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();

    // Use compile-time constants from build script
    let username = env!("WIFI_USERNAME");
    let password = env!("WIFI_PASSWORD");

    // Both operations might use the same endpoint with different modes
    const LOGIN_URL: &str = "https://192.168.254.1:8090/login.xml";
    const LOGOUT_URL: &str = "https://192.168.254.1:8090/login.xml"; // Same endpoint, different mode

    let client = connection_initialisation::LoginClient::new();

    println!("Using embedded credentials from build time");

    match (args.login, args.logout) {
        (true, false) => {
            // Login
            match client.login(username, password, LOGIN_URL).await {
                Ok(response) => {
                    println!("Login attempt completed. Response: {}", response);
                    let _ = Command::new("notify-send")
                        .arg("Wi-Fi Login")
                        .arg(format!("Login attempt for {} completed", username))
                        .output();
                }
                Err(e) => eprintln!("Error: Login Failed with {e}"),
            }
        },
        (false, true) => {
            // Logout
            match client.logout(username, password, LOGOUT_URL).await {
                Ok(response) => {
                    println!("Logout attempt completed. Response: {}", response);
                    let _ = Command::new("notify-send")
                        .arg("Wi-Fi Logout")
                        .arg(format!("Logout attempt for {} completed", username))
                        .output();
                }
                Err(e) => eprintln!("Error: Logout Failed with {e}"),
            }
        },
        (true, true) => {
            eprintln!("Error: Cannot specify both --login and --logout");
        },
        (false, false) => {
            eprintln!("Error: Must specify either --login or --logout");
        },
    }
}

