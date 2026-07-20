name: Build and cross-compile

on:
  push:
  pull_request:
  workflow_dispatch:

jobs:
  build-firmware:
    runs-on: ubuntu-24.04
    steps:
      - name: Check out repository code
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: armv7-unknown-linux-gnueabihf

      - name: Install ARMv7 Cross-Compiler and Dependencies
        run: |
          sudo dpkg --add-architecture armhf
          sudo apt-get update

          sudo apt-get install -y gcc-arm-linux-gnueabihf pkg-config

          sudo apt-get install -y \
              libgstreamer1.0-dev:armhf \
              libgstreamer-plugins-base1.0-dev:armhf \
              libgstreamer-plugins-good1.0-dev:armhf \
              libgstrtspserver-1.0-dev:armhf \
              gstreamer1.0-plugins-base:armhf \
              gstreamer1.0-plugins-good:armhf \
              gstreamer1.0-plugins-bad:armhf \
              gstreamer1.0-plugins-ugly:armhf \
              gstreamer1.0-tools:armhf \
              libglib2.0-dev:armhf

      - name: Build for armv7-unknown-linux-gnueabihf
        run: |
          export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
          
          export PKG_CONFIG_ALLOW_CROSS=1
          export PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig:/usr/share/pkgconfig
          export PKG_CONFIG_SYSROOT_DIR=/
          
          cargo build --release --target armv7-unknown-linux-gnueabihf