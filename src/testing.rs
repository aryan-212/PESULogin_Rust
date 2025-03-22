#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn cli_test() {
        let username = env!("WIFI_USERNAME");
        let password = env!("WIFI_PASSWORD");

        let output = Command::new("cargo")
            .args(["build", "--release"])
            .env("WIFI_USERNAME", "user")
            .env("WIFI_PASSWORD", "pass")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Build successful");
        } else {
            eprintln!("Build failed:\n{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
