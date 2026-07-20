# UVC RTSP Forwarder

A high-performance, low-latency RTSP streaming server written in Rust. It forwards video from a UVC (USB Video Class) camera to RTSP clients using GStreamer under the hood.

Designed specifically to run efficiently on embedded ARM boards (such as Raspberry Pi) by utilizing hardware-accelerated MJPEG output from UVC cameras, minimizing CPU usage and achieving zero-copy payloading.

## Features

- 🚀 **High Performance & Low Latency**: Utilizes GStreamer's zero-copy mechanism (`io-mode=4`) for frame passing.
- ⚡ **Minimal CPU Overhead**: Demands MJPEG directly from the camera hardware (`image/jpeg`), skipping expensive software H.264/JPEG encoding on the CPU.
- 👥 **Shared Streams**: Configured in shared mode (`factory.set_shared(true)`) so multiple connected RTSP clients pull from the same hardware camera stream, avoiding duplicate device reads.
- 🛠️ **Seamless Cross Compilation**: Fully configured with `cross` to target ARMv7 (`armv7-unknown-linux-gnueabihf`) devices.
- 🛑 **Graceful Shutdown**: Automatically clean up and release camera resources upon receiving standard system termination signals (e.g., `Ctrl+C`).

## Prerequisites

The project relies on GStreamer and GLib development libraries.

### Local Development Dependencies

On Debian/Ubuntu-based host systems, install the dependencies using `apt`:

```bash
sudo apt-get update && sudo apt-get install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-good1.0-dev \
    libgstrtspserver-1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-tools \
    libglib2.0-dev
```

## Getting Started

### 1. Local Compilation & Execution

To compile and run the application locally on your development machine:

```bash
# Build the project
cargo build --release

# Run the project (requires an active camera at /dev/video0)
cargo run --release
```

### 2. Configuration

You can configure the application behavior via environment variables:

| Environment Variable | Description | Default Value |
|----------------------|-------------|---------------|
| `CAMERA_DEVICE`      | Path to the camera video device | `/dev/video0` |

For example, to run with a different camera:

```bash
CAMERA_DEVICE=/dev/video1 cargo run --release
```

### 3. Cross-Compiling for ARMv7

This project is pre-configured to be cross-compiled for 32-bit ARM (e.g. Raspberry Pi OS 32-bit, target `armv7-unknown-linux-gnueabihf`) using [cross](https://github.com/cross-rs/cross).

1. Install `cross` on your host system:
   ```bash
   cargo install cross --git https://github.com/cross-rs/cross
   ```

2. Make sure you have Docker running.

3. Run the cross-build command:
   ```bash
   cross build --target armv7-unknown-linux-gnueabihf --release
   ```

   *Note: `Cross.toml` automatically executes [scripts/pre-install.sh](scripts/pre-install.sh) inside the builder container to pull target ARM libraries for compilation.*

4. Find the resulting binary at:
   `target/armv7-unknown-linux-gnueabihf/release/uvc_rtsp_forwarder`

## Usage

Once the server is running, the stream is exposed at:

```text
rtsp://<device-ip>:8554/live
```

You can view the stream using any RTSP-compatible player (like VLC, `ffplay`, or `mpv`).

### Examples

**Using VLC**:
- Open VLC -> Media -> Open Network Stream.
- Enter `rtsp://<device-ip>:8554/live`.

**Using ffplay**:
```bash
ffplay rtsp://<device-ip>:8554/live
```

**Using mpv**:
```bash
mpv rtsp://<device-ip>:8554/live
```

## Architecture Notes

The GStreamer pipeline is constructed dynamically in [src/main.rs](file:///home/qizixi/repos/uvc_rtsp_forwarder/src/main.rs):

```text
( v4l2src device={CAMERA_DEVICE} io-mode=4 ! image/jpeg, width=1280, height=720, framerate=30/1 ! rtpjpegpay name=pay0 pt=96 )
```

- **`v4l2src`**: Video4Linux2 source to interface with the camera.
- **`io-mode=4`**: Selects MMAP (Memory Mapped) user pointer streaming mode, enabling zero-copy transfer of buffers from kernel space.
- **`image/jpeg, width=1280, height=720, framerate=30/1`**: Queries the camera sensor to output MJPEG format natively at 720p 30fps.
- **`rtpjpegpay`**: Payloads the native JPEG stream directly into RFC 2435 RTP packets for transmission.
