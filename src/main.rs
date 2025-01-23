mod connection_initialisation;

#[tokio::main]
async fn main() {
    let client = connection_initialisation::LoginClient::new();

    let result = client
        .login(
            "ADD_YOUR_USERNAME", // the username
            "ADD_YOUR_PASSWORD", // the password
            "https://192.168.254.1:8090/login.xml",
        )
        .await;

    match result {
        Ok(response) => println!("Response: \n{}\nConnected to Wifi", response),
        Err(error) => eprintln!("Error: \n{}\n Connection Failed", error),
    }
}
