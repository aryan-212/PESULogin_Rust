use std::collections::HashMap;

use std::fmt;
use tokio; // Added for async runtime
#[derive(Debug)]
struct CustomError(String);

// Implement Display for the custom error type
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
struct ClientStuff<'a> {
    form_data: HashMap<&'a str, &'a str>,
    client: Option<reqwest::Client>,
}

impl<'a> ClientStuff<'a> {
    fn new() -> Self {
        let client_result = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build();
        match client_result {
            Ok(client) => {
                println!("Client created successfully!");
                Self {
                    form_data: HashMap::new(),
                    client: Some(client),
                }
            }
            Err(e) => {
                eprintln!("Failed to create client: {}", e);
                Self {
                    form_data: HashMap::new(),
                    client: None,
                }
            }
        }
    }

    pub async fn send_request(
        &mut self,
        username: &'a str,
        password: &'a str,
        cie_login_url: &str,
    ) -> Result<String, CustomError> {
        self.form_data.insert("mode", "191");
        self.form_data.insert("username", username);
        self.form_data.insert("password", password);
        self.form_data.insert("a", "1713188925839");
        self.form_data.insert("producttype", "0");

        if let Some(client) = &self.client {
            match client
                .post(cie_login_url)
                .form(&self.form_data)
                .send()
                .await
            {
                Ok(response) => match response.text().await {
                    Ok(text) => Ok(text),
                    Err(_) => Err(CustomError("Failed to read response text".to_string())),
                },
                Err(_) => Err(CustomError("Request failed".to_string())),
            }
        } else {
            Err(CustomError("HTTP client is not initialized".to_string()))
        }
    }
}

#[tokio::main]
async fn main() {
    let username = "parallax04";
    let password = "gawarbench";
    let cie_login_url = "https://192.168.254.1:8090/login.xml";

    let mut client_stuff = ClientStuff::new();
    match client_stuff
        .send_request(username, password, cie_login_url)
        .await
    {
        Ok(response_text) => {
            println!("Response body: {}", response_text);
        }
        Err(e) => {
            eprintln!("Error sending request: {}", e);
        }
    }
}
