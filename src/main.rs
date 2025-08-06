use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;
use std::process::Command;
mod connection_initialisation;

#[derive(Parser, Debug)]
#[command(name = "pesl", about = "Wi-Fi Login/Logout CLI")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Logs in to PESU Wi-Fi
    Login {
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
    },
    /// Logs out from PESU Wi-Fi
    Logout {
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
    },
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();

    // Use compile-time constants from build script

    // Both operations use the same endpoint with different modes
    const LOGIN_URL: &str = "https://192.168.254.1:8090/login.xml";
    const LOGOUT_URL: &str = "https://192.168.254.1:8090/logout.xml";

    let client = connection_initialisation::LoginClient::new();

    println!("Using embedded credentials from build time");

    match args.command {
        Commands::Login { username, password } => {
            if let (Some(name), Some(pass)) = (username, password) {
                match client.login(&name, &pass, LOGIN_URL).await {
                    Ok(response) => {
                        println!("Login attempt completed. Response: {}", response);
                        notify_user(
                            "Wi-Fi Login",
                            &format!("Login attempt for {} completed", name),
                        );
                    }
                    Err(e) => eprintln!("Error: Login failed with {}", e),
                }
            } else {
                println!("Using the embedded login creds");
                let name = env::var("WIFI_USERNAME").unwrap();
                let pass = env::var("WIFI_PASSWORD").unwrap();
                match client.login(&name, &pass, LOGIN_URL).await {
                    Ok(response) => {
                        println!("Login attempt completed. Response: {}", response);
                        notify_user(
                            "Wi-Fi Login",
                            &format!("Login attempt for {} completed", name),
                        );
                    }
                    Err(e) => eprintln!("Error: Login failed with {}", e),
                }
            }
        }
        Commands::Logout { username, password } => {
            if let (Some(name), Some(pass)) = (username, password) {
                match client.logout(&name, &pass, LOGOUT_URL).await {
                    Ok(response) => {
                        println!("Login attempt completed. Response: {}", response);
                        notify_user(
                            "Wi-Fi Login",
                            &format!("Login attempt for {} completed", name),
                        );
                    }
                    Err(e) => eprintln!("Error: Login failed with {}", e),
                }
            } else {
                println!("Using the embedded login creds");
                let name = env::var("WIFI_USERNAME").unwrap();
                let pass = env::var("WIFI_PASSWORD").unwrap();
                match client.logout(&name, &pass, LOGOUT_URL).await {
                    Ok(response) => {
                        println!("Login attempt completed. Response: {}", response);
                        notify_user(
                            "Wi-Fi Login",
                            &format!("Login attempt for {} completed", name),
                        );
                    }
                    Err(e) => eprintln!("Error: Login failed with {}", e),
                }
            }
        }
    }
}

/// Send a desktop notification using notify-send
fn notify_user(title: &str, message: &str) {
    let _ = Command::new("notify-send").arg(title).arg(message).output();
}
