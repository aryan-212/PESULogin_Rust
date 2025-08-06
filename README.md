# PESULogin_Rust

An _oxidised_ version of [PESUAutoLogin](https://github.com/aryan-212/PESU_AutoLogin) in a small executable.

## Installation

```bash
git clone https://github.com/aryan-212/PESULogin_Rust.git
cd PESULogin_Rust/
WIFI_USERNAME=user WIFI_PASSWORD=pass cargo build --release
sudo cp $HOME/PESULogin_Rust/target/release/pesu_login_test /usr/bin/pesulogin
```

## Usage

### Logging in

```bash
pesulogin login
# Or if there is no WIFI_USERNAME and WIFI_PASSWORD env vars
pesulogin login --username "WIFI_USERNAME" --password "WIFI_PASSWORD"
```

### Logging out

```bash
pesulogin logout
# Or if there is no WIFI_USERNAME and WIFI_PASSWORD env vars
pesulogin logout --username "WIFI_USERNAME" --password "WIFI_PASSWORD"

```
