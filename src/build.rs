use std::env;

fn main() {
    // Get credentials from environment variables and pass them to the compiler
    if let Ok(username) = env::var("WIFI_USERNAME") {
        println!("cargo:rustc-env=WIFI_USERNAME={}", username);
    } else {
        eprintln!("Warning: WIFI_USERNAME not set, using default");
        println!("cargo:rustc-env=WIFI_USERNAME=default_user");
    }

    if let Ok(password) = env::var("WIFI_PASSWORD") {
        println!("cargo:rustc-env=WIFI_PASSWORD={}", password);
    } else {
        eprintln!("Warning: WIFI_PASSWORD not set, using default");
        println!("cargo:rustc-env=WIFI_PASSWORD=default_pass");
    }

    // Tell cargo to rerun if these environment variables change
    println!("cargo:rerun-if-env-changed=WIFI_USERNAME");
    println!("cargo:rerun-if-env-changed=WIFI_PASSWORD");
}