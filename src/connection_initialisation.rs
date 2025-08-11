use reqwest::Error;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LoginClient {
    pub client: reqwest::Client,
}

fn get_sys_time() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

fn build_form_data(mode: &str, username: &str, password: &str) -> HashMap<String, String> {
    HashMap::from([
        ("mode".into(), mode.into()),
        ("username".into(), username.into()),
        ("password".into(), password.into()),
        ("a".into(), get_sys_time()),
        ("producttype".into(), "0".into()),
    ])
}

impl LoginClient {
    pub fn handle_action(username: &str, password: &str) -> Result<(), Error> {
        Ok(())
    }
 
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create client");

        Self { client }
    }

    pub async fn handle_request(
        &self,
        is_login: bool,
        username: Option<&str>,
        password: Option<&str>,
        login_url: &str,
        logout_url: &str,
    ) -> Result<String, Error> {
        let (name, pass) = match (username, password) {
            (Some(name), Some(pass)) => (name.to_string(), pass.to_string()),
            (None, None) => {
                // Use environment variables if no credentials provided
                let name = std::env::var("WIFI_USERNAME")
                    .expect("WIFI_USERNAME environment variable not set");
                let pass = std::env::var("WIFI_PASSWORD")
                    .expect("WIFI_PASSWORD environment variable not set");
                (name, pass)
            }
            _ => {
                panic!("Both username and password must be provided, or neither");
            }
        };

        if is_login {
            self.login(&name, &pass, login_url).await
        } else {
            self.logout(&name, &pass, logout_url).await
        }
    }

    pub async fn login(&self, username: &str, password: &str, url: &str) -> Result<String, Error> {
        let res = self
            .client
            .post(url)
            .form(&build_form_data("191", username, password))
            .send()
            .await?
            .text()
            .await?;

        println!("Login Response: {}", res);
        Ok(res)
    }

    pub async fn logout(&self, username: &str, password: &str, url: &str) -> Result<String, Error> {
        let res = self
            .client
            .post(url)
            .form(&build_form_data("193", username, password))
            .send()
            .await?
            .text()
            .await?;

        println!("Logout Response: {}", res);
        Ok(res)
    }
}
