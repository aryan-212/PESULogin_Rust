use std::env;
mod connection_initialisation;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let client = connection_initialisation::LoginClient::new();

    let result = client
        .login(
            &args[1],
            &args[2], // the password
            "https://192.168.254.1:8090/login.xml",
        )
        .await;

    match result {
        Ok(response) => println!("Response: \n{}\nConnected to Wifi", response),
        Err(error) => eprintln!("Error: \n{}\n Connection Failed", error),
    }
}
