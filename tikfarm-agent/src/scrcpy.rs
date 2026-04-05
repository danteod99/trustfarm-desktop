use crate::adb;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::TcpListener as StdTcpListener;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub struct ScrcpyManager {
    pub adb_path: PathBuf,
    pub scrcpy_server_path: PathBuf,
    sessions: Arc<Mutex<HashMap<String, ScrcpySession>>>,
    scid_counter: Arc<Mutex<u32>>,
}

struct ScrcpySession {
    _process: Child,
    scid: u32,
}

impl ScrcpyManager {
    pub fn new(adb_path: PathBuf, scrcpy_server_path: PathBuf) -> Self {
        Self {
            adb_path,
            scrcpy_server_path,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            scid_counter: Arc::new(Mutex::new(1)),
        }
    }

    /// Start scrcpy for a device using adb reverse with localabstract socket.
    /// Returns (video_stream, control_stream)
    pub async fn start_scrcpy(
        &self,
        serial: &str,
        max_size: u32,
        fps: u32,
    ) -> Result<(TcpStream, TcpStream), String> {
        // Kill old session
        {
            let mut sessions = self.sessions.lock().await;
            if let Some(mut old) = sessions.remove(serial) {
                let _ = old._process.kill();
                let _ = Command::new(&self.adb_path)
                    .args(["-s", serial, "reverse", "--remove-all"])
                    .output();
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
        }

        // Get unique scid
        let scid = {
            let mut counter = self.scid_counter.lock().await;
            let s = *counter;
            *counter += 1;
            s
        };
        let scid_hex = format!("{:08x}", scid);

        // Push scrcpy-server
        adb::push_scrcpy_server(&self.adb_path, serial, &self.scrcpy_server_path)?;

        // Listen on random local port
        let listener = StdTcpListener::bind("127.0.0.1:0")
            .map_err(|e| format!("Failed to bind: {}", e))?;
        let local_port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).ok();

        // Create adb reverse: localabstract:scrcpy_SCID -> tcp:LOCAL_PORT
        let reverse_output = Command::new(&self.adb_path)
            .args([
                "-s", serial, "reverse",
                &format!("localabstract:scrcpy_{}", scid_hex),
                &format!("tcp:{}", local_port),
            ])
            .output()
            .map_err(|e| format!("adb reverse failed: {}", e))?;

        if !reverse_output.status.success() {
            return Err(format!("adb reverse failed: {}",
                String::from_utf8_lossy(&reverse_output.stderr)));
        }

        log::info!("Created reverse tunnel for {} scid={} port={}", serial, scid_hex, local_port);

        // Start scrcpy-server on device (without tunnel_forward, uses reverse connection)
        let server_cmd = format!(
            "CLASSPATH=/data/local/tmp/scrcpy-server.jar app_process / com.genymobile.scrcpy.Server 3.1 scid={} video=true audio=false control=true max_size={} max_fps={} video_codec=h264",
            scid_hex, max_size, fps
        );

        let process = Command::new(&self.adb_path)
            .args(["-s", serial, "shell", &server_cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start scrcpy: {}", e))?;

        log::info!("Started scrcpy-server for {} (max_size={}, fps={})", serial, max_size, fps);

        // Convert std listener to tokio and accept connections
        let tokio_listener = TcpListener::from_std(listener)
            .map_err(|e| format!("Failed to convert listener: {}", e))?;

        // Accept video connection (first)
        let (video_stream, _) = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            tokio_listener.accept()
        ).await
            .map_err(|_| "Timeout waiting for video connection".to_string())?
            .map_err(|e| format!("Video accept failed: {}", e))?;

        log::info!("Video socket connected for {}", serial);

        // Accept control connection (second)
        let (control_stream, _) = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            tokio_listener.accept()
        ).await
            .map_err(|_| "Timeout waiting for control connection".to_string())?
            .map_err(|e| format!("Control accept failed: {}", e))?;

        log::info!("Control socket connected for {}", serial);

        // Save session
        {
            let mut sessions = self.sessions.lock().await;
            sessions.insert(serial.to_string(), ScrcpySession {
                _process: process,
                scid,
            });
        }

        Ok((video_stream, control_stream))
    }
}

/// Convert JSON touch/key events from frontend to scrcpy binary control protocol
fn convert_event_to_scrcpy(evt: &serde_json::Value, screen_w: u32, screen_h: u32) -> Option<Vec<u8>> {
    let evt_type = evt.get("type")?.as_str()?;

    match evt_type {
        "touch" => {
            let operation = evt.get("operation")?.as_str()?;
            let x_ratio: f64 = evt.get("x")?.as_str()?.parse().ok()?;
            let y_ratio: f64 = evt.get("y")?.as_str()?.parse().ok()?;

            let x = (x_ratio * screen_w as f64) as u32;
            let y = (y_ratio * screen_h as f64) as u32;

            // scrcpy inject touch event (type 2):
            // u8 type (2) + u8 action + u64 pointer_id + u32 x + u32 y + u16 w + u16 h + u16 pressure + u32 action_button + u32 buttons
            let action: u8 = match operation {
                "d" => 0, // ACTION_DOWN
                "u" => 1, // ACTION_UP
                "m" => 2, // ACTION_MOVE
                _ => return None,
            };

            let pressure: u16 = if action == 1 { 0 } else { 0xFFFF };

            let mut buf = Vec::with_capacity(32);
            buf.push(2u8); // SC_CONTROL_MSG_TYPE_INJECT_TOUCH_EVENT
            buf.push(action);
            buf.extend_from_slice(&0u64.to_be_bytes()); // pointer_id
            buf.extend_from_slice(&(x as i32).to_be_bytes()); // x
            buf.extend_from_slice(&(y as i32).to_be_bytes()); // y
            buf.extend_from_slice(&(screen_w as u16).to_be_bytes()); // screen width
            buf.extend_from_slice(&(screen_h as u16).to_be_bytes()); // screen height
            buf.extend_from_slice(&pressure.to_be_bytes()); // pressure
            buf.extend_from_slice(&1u32.to_be_bytes()); // action_button (primary)
            buf.extend_from_slice(&0u32.to_be_bytes()); // buttons

            Some(buf)
        }
        "key" | "keycode" => {
            let keycode: u32 = evt.get("keycode")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            let action: u8 = match evt.get("operation").and_then(|v| v.as_str()).unwrap_or("d") {
                "d" => 0, // ACTION_DOWN
                "u" => 1, // ACTION_UP
                _ => 0,
            };

            let mut buf = Vec::with_capacity(14);
            buf.push(0u8); // SC_CONTROL_MSG_TYPE_INJECT_KEYCODE
            buf.push(action);
            buf.extend_from_slice(&keycode.to_be_bytes());
            buf.extend_from_slice(&0u32.to_be_bytes()); // repeat
            buf.extend_from_slice(&0u32.to_be_bytes()); // metastate

            Some(buf)
        }
        "screenMode" => {
            // Screen on/off - send device power key
            let mode = evt.get("mode").and_then(|v| v.as_str()).unwrap_or("on");
            if mode == "off" {
                // SC_CONTROL_MSG_TYPE_SET_SCREEN_POWER_MODE = 10
                let mut buf = Vec::with_capacity(2);
                buf.push(10u8);
                buf.push(0u8); // OFF
                Some(buf)
            } else {
                let mut buf = Vec::with_capacity(2);
                buf.push(10u8);
                buf.push(2u8); // NORMAL
                Some(buf)
            }
        }
        _ => None,
    }
}

fn find_free_port() -> Option<u16> {
    StdTcpListener::bind("127.0.0.1:0").ok().map(|l| l.local_addr().unwrap().port())
}

pub async fn run_ws_server(
    scrcpy_manager: Arc<ScrcpyManager>,
    ws_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", ws_port)).await?;
    log::info!("Scrcpy WebSocket server listening on port {}", ws_port);

    loop {
        let (stream, addr) = listener.accept().await?;
        let manager = scrcpy_manager.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_ws_connection(stream, manager).await {
                log::error!("WS error from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_ws_connection(
    stream: TcpStream,
    manager: Arc<ScrcpyManager>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Receive handshake from frontend: serial, max_size, control, fps
    let serial = match ws_receiver.next().await {
        Some(Ok(Message::Text(s))) => s.trim().to_string(),
        _ => return Err("Expected serial".into()),
    };
    log::info!("WS client for device: {}", serial);

    let max_size: u32 = match ws_receiver.next().await {
        Some(Ok(Message::Text(s))) => s.trim().parse().unwrap_or(800),
        _ => 800,
    };
    let _control = ws_receiver.next().await;
    let fps: u32 = match ws_receiver.next().await {
        Some(Ok(Message::Text(s))) => s.trim().parse().unwrap_or(15),
        _ => 15,
    };

    // Start scrcpy and get video+control streams
    let (video_stream, _control_stream) = manager.start_scrcpy(&serial, max_size, fps).await?;
    let (mut video_reader, _video_writer) = video_stream.into_split();

    // Read scrcpy v3.1 video stream header (no dummy byte in v3):
    // Device name (64 bytes, null-padded)
    let mut name_buf = [0u8; 64];
    video_reader.read_exact(&mut name_buf).await
        .map_err(|e| format!("Failed to read device name: {}", e))?;
    let device_name = String::from_utf8_lossy(&name_buf)
        .trim_end_matches('\0')
        .to_string();

    // Codec info: codec_name (4 bytes) + width (u32 BE) + height (u32 BE)
    let mut codec_buf = [0u8; 12];
    video_reader.read_exact(&mut codec_buf).await
        .map_err(|e| format!("Failed to read codec info: {}", e))?;

    let width = u32::from_be_bytes([codec_buf[4], codec_buf[5], codec_buf[6], codec_buf[7]]);
    let height = u32::from_be_bytes([codec_buf[8], codec_buf[9], codec_buf[10], codec_buf[11]]);

    log::info!("Streaming {} {}x{} for {}", device_name, width, height, serial);

    // Send handshake to frontend
    ws_sender.send(Message::Text(device_name)).await?;
    ws_sender.send(Message::Text(format!("{}x{}", width, height))).await?;

    // Get control writer for forwarding input events
    let (_control_reader, mut control_writer) = _control_stream.into_split();

    // Stream complete frames from scrcpy to WebSocket
    // Each frame: 12-byte header (PTS u64 BE + packet_size u32 BE) + H264 data
    // Frontend expects each WS message to be exactly one complete frame
    let serial_clone = serial.clone();
    let video_task = tokio::spawn(async move {
        let mut header_buf = [0u8; 12];
        loop {
            // Read 12-byte frame header
            if let Err(_) = video_reader.read_exact(&mut header_buf).await {
                break;
            }

            let packet_size = u32::from_be_bytes([
                header_buf[8], header_buf[9], header_buf[10], header_buf[11]
            ]) as usize;

            if packet_size > 4_000_000 {
                log::error!("Abnormal packet size {} for {}", packet_size, serial_clone);
                break;
            }

            // Read H264 packet data
            let mut packet = vec![0u8; packet_size];
            if let Err(_) = video_reader.read_exact(&mut packet).await {
                break;
            }

            // Combine header + data into single WS message
            let mut frame = Vec::with_capacity(12 + packet_size);
            frame.extend_from_slice(&header_buf);
            frame.extend_from_slice(&packet);

            if ws_sender.send(Message::Binary(frame)).await.is_err() {
                break;
            }
        }
    });

    // Handle control messages from frontend
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                use tokio::io::AsyncWriteExt;
                let _ = control_writer.write_all(&data).await;
            }
            Ok(Message::Text(text)) => {
                if let Ok(evt) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(binary) = convert_event_to_scrcpy(&evt, width, height) {
                        use tokio::io::AsyncWriteExt;
                        let _ = control_writer.write_all(&binary).await;
                    }
                }
            }
            Ok(Message::Close(_)) | Err(_) => break,
            _ => {}
        }
    }

    video_task.abort();
    log::info!("Connection closed for {}", serial);

    Ok(())
}
