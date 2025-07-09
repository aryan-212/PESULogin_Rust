use std::collections::HashMap;

use reqwest::Error;

pub struct LoginClient {
    pub client: reqwest::Client,
}

impl LoginClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build();

        match client {
            Ok(client) => Self { client },
            Err(e) => panic!("Failed to create client: {}", e),
        }
    }

    pub async fn login(&self, username: &str, password: &str, url: &str) -> Result<String, Error> {
        let mut form_data = HashMap::new();
        form_data.insert("mode", "191");
        form_data.insert("username", username);
        form_data.insert("password", password);
        form_data.insert("a", "1713188925839");
        form_data.insert("producttype", "0");

        self.client
            .post(url)
            .form(&form_data)
            .send()
            .await?
            .text()
            .await
    }
}
