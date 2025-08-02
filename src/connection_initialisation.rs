use reqwest::Error;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
            
        let mut form_data = HashMap::new();
        form_data.insert("mode", "191");
        form_data.insert("username", username);
        form_data.insert("password", password);
        form_data.insert("a", &millis);
        form_data.insert("producttype", "0");

        let res = self.client
            .post(url)
            .form(&form_data)
            .send()
            .await?
            .text()
            .await?;
        println!("Login Response: {}", res);
        Ok(res)
    }
    
    pub async fn logout(&self, username: &str, password: &str, url: &str) -> Result<String, Error> {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
            
        let mut form_data = HashMap::new();
        form_data.insert("mode", "193"); // Different mode for logout
        form_data.insert("username", username);
        form_data.insert("password", password);
        form_data.insert("a", &millis);
        form_data.insert("producttype", "0");
        
        let res = self
            .client
            .post(url)
            .form(&form_data)
            .send()
            .await?
            .text()
            .await?;
        println!("Logout Response: {}", res);
        Ok(res)
    }
}
