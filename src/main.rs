use dotenv::dotenv;
use std::env;
use std::process::Command;
mod build;
mod connection_initialisation;
mod testing;

#[tokio::main]
async fn main() {
    // Read Wi-Fi credentials at runtime
    dotenv().ok();

    let username = env::var("WIFI_USERNAME").unwrap_or_else(|_| "UnknownUser".to_string());
    let password = env::var("WIFI_PASSWORD").unwrap_or_else(|_| "UnknownPassword".to_string());

    println!("Using username: {}", username); // Debugging

    // Initialize the login client
    let client = connection_initialisation::LoginClient::new();

    // Log in to the Wi-Fi
    let result = client
        .login(
            &username, // Use the runtime username
            &password, // Use the runtime password
            "https://192.168.254.1:8090/login.xml",
        )
        .await;

    // Handle the login result
    match result {
        Ok(_) => {
            let output = Command::new("notify-send")
                .arg("Wi-Fi Connected")
                .arg(format!("Connected to {} successfully", username))
                .output();

            if output.is_err() {
                eprintln!("Failed to send notification");
            }
        }
        Err(_) => eprintln!("Error:\nConnection Failed"),
    }
}
