use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;
use std::process::Command;

mod build;
mod connection_initialisation;

#[derive(Parser, Debug)]
#[command(name = "pesl", about = "Wi-Fi Login/Logout CLI")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login to Wi-Fi
    Login,
    /// Logout from Wi-Fi
    Logout,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();

    // Use compile-time constants from build script
    let username = env!("WIFI_USERNAME");
    let password = env!("WIFI_PASSWORD");

    // Both operations use the same endpoint with different modes
    const LOGIN_URL: &str = "https://192.168.254.1:8090/login.xml";
    const LOGOUT_URL: &str = "https://192.168.254.1:8090/logout.xml";

    let client = connection_initialisation::LoginClient::new();

    println!("Using embedded credentials from build time");

    match args.command {
        Commands::Login => match client.login(username, password, LOGIN_URL).await {
            Ok(response) => {
                println!("Login attempt completed. Response: {}", response);
                notify_user(
                    "Wi-Fi Login",
                    &format!("Login attempt for {} completed", username),
                );
            }
            Err(e) => eprintln!("Error: Login failed with {}", e),
        },
        Commands::Logout => match client.logout(username, password, LOGOUT_URL).await {
            Ok(response) => {
                println!("Logout attempt completed. Response: {}", response);
                notify_user(
                    "Wi-Fi Logout",
                    &format!("Logout attempt for {} completed", username),
                );
            }
            Err(e) => eprintln!("Error: Logout failed with {}", e),
        },
    }
}

/// Send a desktop notification using notify-send
fn notify_user(title: &str, message: &str) {
    let _ = Command::new("notify-send").arg(title).arg(message).output();
}
