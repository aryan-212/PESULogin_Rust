
FROM rust:latest

# Enable multi-arch support and update package lists
RUN dpkg --add-architecture arm64 && \
    apt update && apt upgrade -y && \
    apt install -y \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    libssl-dev:arm64 \
    pkg-config \
    apt-utils

# Add the ARM64 target for Rust and install the appropriate toolchain
RUN rustup target add aarch64-unknown-linux-gnu && \
    rustup toolchain install stable-aarch64-unknown-linux-gnu

# Set the working directory
WORKDIR /app

# Environment variables for cross-compilation
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    PKG_CONFIG_ALLOW_CROSS=1 \
    PKG_CONFIG_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig" \
    PKG_CONFIG_SYSROOT_DIR="/" \
    OPENSSL_DIR="/usr/lib/aarch64-linux-gnu" \
    OPENSSL_LIB_DIR="/usr/lib/aarch64-linux-gnu" \
    OPENSSL_INCLUDE_DIR="/usr/include/aarch64-linux-gnu" \
    PKG_CONFIG=pkg-config

# Default command to build the Rust project for ARM64
CMD ["cargo", "build", "--target", "aarch64-unknown-linux-gnu"]
