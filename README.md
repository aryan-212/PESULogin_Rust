# PESULogin_Rust

An _oxidised_ version of [PESUAutoLogin](https://github.com/aryan-212/PESU_AutoLogin) in a small executable.

## Installation
Replace `user` and `pass` with actual usename and password respectively
```bash
git clone https://github.com/aryan-212/PESULogin_Rust.git
cd PESULogin_Rust/
WIFI_USERNAME=user WIFI_PASSWORD=pass cargo build --release
sudo cp $HOME/PESULogin_Rust/target/debug/pesu_login_test /usr/bin/pesulogin
```

## Usage

```bash
pesulogin
```
