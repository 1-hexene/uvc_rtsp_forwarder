#!/bin/bash
# set -e

# 1. 切换为 32位 armhf 架构
dpkg --add-architecture armhf
echo "running apt update"
apt-get update
echo "installing dependencies"

# 2. 安装编译所需的 32位 依赖库（包含 gstreamer 核心、插件以及 RTSP 服务器组件）
apt-get install -y \
    qemu-user-static \
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