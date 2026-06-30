// use gst::prelude::*;
use gst_rtsp_server::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化 GStreamer 运行时
    gst::init()?;

    // 2. 创建 RTSP 服务器实例
    let server = gst_rtsp_server::RTSPServer::new();
    // 设置监听端口，默认 8554
    server.set_service("8554");

    // 3. 获取并设置媒体映射（Media Mapping）
    let mounts = server.mount_points().ok_or("无法获取 RTSP 挂载点")?;
    let factory = gst_rtsp_server::RTSPMediaFactory::new();

    // 4. 构建高性能零拷贝流水线 (Pipeline)
    // 解释：
    // - v4l2src: 直接读取 UVC 摄像头
    // - image/jpeg: 强制让摄像头硬件输出 mjpeg，不经过 CPU 解码
    // - rtpjpegpay: 将 JPEG 帧打包为 RTP 包（符合 RFC 2435 标准，零拷贝）
    let device = env::var("CAMERA_DEVICE").unwrap_or_else(|_| "/dev/video0".to_string());
    let width = 1280;
    let height = 720;
    let fps = 30;

    let launch_description = format!(
    "( v4l2src device={} io-mode=4 extra-controls=\"c,focus_auto=0,focus_absolute=0\" ! \
       image/jpeg, width={}, height={}, framerate={}/1 ! \
       rtpjpegpay name=pay0 pt=96 )",
    device, width, height, fps
);

    println!("GStreamer 管道配置: {}", launch_description);
    factory.set_launch(&launch_description);
    
    // 设置为共享模式：多个客户端连接时，复用同一个摄像头数据流，避免多次打开设备
    factory.set_shared(true);

    // 5. 挂载到指定的路径（例如: rtsp://<IP>:8554/live）
    mounts.add_factory("/live", factory);

    // 6. 启动服务器
    let _id = server.attach(None).map_err(|_| "无法启动 RTSP 服务器")?;

    println!("🚀 高性能 UVC-RTSP 转发服务已启动！");
    println!("🔗 播放地址: rtsp://<你的板子IP>:8554/live");

    // 7. 保持主循环运行（GStreamer 基于 GLib MainLoop）
    let main_loop = glib::MainLoop::new(None, false);
    
    // 在异步环境中阻塞运行 GLib 主循环
    let main_loop_clone = main_loop.clone();
    tokio::task::spawn_blocking(move || {
        main_loop_clone.run();
    });

    // 监听退出信号
    tokio::signal::ctrl_c().await?;
    println!("正在关闭服务器...");
    main_loop.quit();

    Ok(())
}