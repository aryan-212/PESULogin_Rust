use std::env;
mod build;
mod connection_initialisation;

#[tokio::main]
async fn main() {
    // Read Wi-Fi credentials embedded at compile time
    let username = env!("WIFI_USERNAME");
    let password = env!("WIFI_PASSWORD");

    // Initialize the login client
    let client = connection_initialisation::LoginClient::new();

    // Log in to the Wi-Fi
    let result = client
        .login(
            username, // Use the embedded username
            password, // Use the embedded password
            "https://192.168.254.1:8090/login.xml",
        )
        .await;

    // Handle the login result
    match result {
        Ok(_) => println!("Response:\nConnected to Wifi, via {}", username),
        Err(_) => eprintln!("Error:\n Connection Failed"),
    }
}
