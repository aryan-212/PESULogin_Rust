use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let username = env::var("WIFI_USERNAME").unwrap_or_else(|_| "default_user".to_string());
    let password = env::var("WIFI_PASSWORD").unwrap_or_else(|_| "default_pass".to_string());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wifi_config.rs");

    fs::write(
        &dest_path,
        format!(
            r#"
            pub const WIFI_USERNAME: &str = "{username}";
            pub const WIFI_PASSWORD: &str = "{password}";
            "#,
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-env-changed=WIFI_USERNAME");
    println!("cargo:rerun-if-env-changed=WIFI_PASSWORD");
}
