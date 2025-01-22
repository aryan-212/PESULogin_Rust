mod connection_initialisation;
use crate::connection_initialisation::*;
use reqwest;

#[tokio::main]
async fn main() {
    let client = connection_initialisation::LoginClient::new();

    let result = client
        .login(
            "parallax04", // the username
            "gawarbench", // the password
            "https://192.168.254.1:8090/login.xml",
        )
        .await;

    match result {
        Ok(response) => println!("Response: \n{}\nConnected to Wifi", response),
        Err(error) => eprintln!("Error: \n{}\n Connection Failed", error),
    }
}
