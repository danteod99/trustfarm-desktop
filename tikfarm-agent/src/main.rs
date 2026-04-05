mod adb;
mod scrcpy;

use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use warp::Filter;

const API_PORT: u16 = 50809;
const WS_PORT: u16 = 51627;

fn find_scrcpy_server() -> PathBuf {
    let candidates = vec![
        Some(PathBuf::from(r"C:\Users\Usuario\tikfarm-agent\scrcpy-dist\scrcpy-win64-v3.1\scrcpy-server")),
        dirs::config_dir().map(|d| d.join("com.tikfarm").join("bin").join("scrcpy-server")),
        dirs::config_dir().map(|d| d.join("com.tikmatrix").join("bin").join("scrcpy-server")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if candidate.exists() {
            log::info!("Found scrcpy-server at: {:?}", candidate);
            return candidate;
        }
    }
    PathBuf::from("scrcpy-server")
}

fn write_port_file(filename: &str, port: u16) {
    let app_data = dirs::config_dir().unwrap_or_default().join("com.tikfarm");
    std::fs::create_dir_all(&app_data).ok();
    let path = app_data.join(filename);
    std::fs::write(&path, port.to_string()).ok();
    log::info!("Wrote {} = {}", filename, port);
}

/// Run ADB command on multiple serials
fn run_on_serials(adb_path: &PathBuf, body: &serde_json::Value, args: &[&str]) -> serde_json::Value {
    let serials = body["serials"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
        .unwrap_or_default();

    if serials.is_empty() {
        // Single serial
        if let Some(serial) = body["serial"].as_str() {
            let result = adb::run_adb_command(adb_path, serial, args);
            return json!({ "code": 200, "data": result.unwrap_or_default() });
        }
        return json!({ "code": 400, "error": "No serials provided" });
    }

    let mut results = Vec::new();
    for serial in &serials {
        let result = adb::run_adb_command(adb_path, serial, args);
        results.push(json!({
            "serial": serial,
            "success": result.is_ok(),
            "output": result.unwrap_or_else(|e| e)
        }));
    }
    json!({ "code": 200, "data": results })
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("TikFarm Agent v2 starting (standalone)...");

    let adb_path = adb::find_adb();
    log::info!("ADB: {:?}", adb_path);

    let scrcpy_server = find_scrcpy_server();
    log::info!("scrcpy-server: {:?}", scrcpy_server);

    write_port_file("port.txt", API_PORT);
    write_port_file("wsport.txt", WS_PORT);

    let scrcpy_manager = Arc::new(scrcpy::ScrcpyManager::new(adb_path.clone(), scrcpy_server));

    let ws_manager = scrcpy_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = scrcpy::run_ws_server(ws_manager, WS_PORT).await {
            log::error!("WS server error: {}", e);
        }
    });

    // === DEVICE ROUTES ===
    let adb1 = adb_path.clone();
    let devices_route = warp::path!("api" / "device")
        .and(warp::get())
        .map(move || {
            let devices = adb::list_devices(&adb1);
            warp::reply::json(&json!({ "data": devices }))
        });

    let adb_idx = adb_path.clone();
    let index_route = warp::path!("api" / "device" / "index")
        .and(warp::get())
        .map(move || {
            let devices = adb::list_devices(&adb_idx);
            warp::reply::json(&json!({ "data": devices }))
        });

    let adb_count = adb_path.clone();
    let count_route = warp::path!("api" / "device" / "count_online")
        .and(warp::get())
        .map(move || {
            let count = adb::list_devices(&adb_count).len();
            warp::reply::json(&json!({ "data": count }))
        });

    // === ADB COMMAND (parallel execution) ===
    let adb_cmd = adb_path.clone();
    let adb_cmd_route = warp::path!("agent" / "api" / "adb_command")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            let serials: Vec<String> = if let Some(arr) = body["serials"].as_array() {
                arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
            } else if let Some(s) = body["serial"].as_str() {
                vec![s.to_string()]
            } else {
                vec![]
            };

            let args: Vec<String> = if let Some(arr) = body["args"].as_array() {
                arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
            } else if let Some(cmd) = body["command"].as_str() {
                cmd.split_whitespace().map(String::from).collect()
            } else {
                vec![]
            };

            log::info!("ADB command on {} devices: {:?}", serials.len(), args);

            // Execute in parallel using threads
            let handles: Vec<_> = serials.iter().map(|serial| {
                let adb = adb_cmd.clone();
                let serial = serial.clone();
                let args = args.clone();
                std::thread::spawn(move || {
                    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                    let result = adb::run_adb_command(&adb, &serial, &args_ref);
                    json!({
                        "serial": serial,
                        "success": result.is_ok(),
                        "output": result.unwrap_or_else(|e| e)
                    })
                })
            }).collect();

            let results: Vec<_> = handles.into_iter()
                .filter_map(|h| h.join().ok())
                .collect();

            warp::reply::json(&json!({ "code": 200, "data": results }))
        });

    // === TIKTOK COMMANDS ===
    let adb_open = adb_path.clone();
    let open_tiktok_route = warp::path!("api" / "open_tiktok")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            log::info!("Opening TikTok on devices");
            let result = run_on_serials(&adb_open, &body, &[
                "shell", "monkey", "-p", "com.zhiliaoapp.musically", "-c", "android.intent.category.LAUNCHER", "1"
            ]);
            warp::reply::json(&result)
        });

    let adb_stop = adb_path.clone();
    let stop_tiktok_route = warp::path!("api" / "stop_tiktok")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            log::info!("Stopping TikTok on devices");
            let result = run_on_serials(&adb_stop, &body, &[
                "shell", "am", "force-stop", "com.zhiliaoapp.musically"
            ]);
            warp::reply::json(&result)
        });

    // === CLEAR DATA ===
    let adb_clear = adb_path.clone();
    let clear_data_route = warp::path!("api" / "clear_data")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            log::info!("Clearing TikTok data on devices");
            let result = run_on_serials(&adb_clear, &body, &[
                "shell", "pm", "clear", "com.zhiliaoapp.musically"
            ]);
            warp::reply::json(&result)
        });

    // === GRANT TIKTOK PERMISSIONS ===
    let adb_grant = adb_path.clone();
    let grant_tiktok_route = warp::path!("api" / "grant_tiktok")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            log::info!("Granting TikTok permissions");
            let serials = body["serials"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();

            let permissions = vec![
                "android.permission.CAMERA",
                "android.permission.RECORD_AUDIO",
                "android.permission.READ_EXTERNAL_STORAGE",
                "android.permission.WRITE_EXTERNAL_STORAGE",
                "android.permission.ACCESS_FINE_LOCATION",
            ];

            let mut results = Vec::new();
            for serial in &serials {
                for perm in &permissions {
                    let _ = adb::run_adb_command(&adb_grant, serial, &[
                        "shell", "pm", "grant", "com.zhiliaoapp.musically", perm
                    ]);
                }
                results.push(json!({ "serial": serial, "success": true }));
            }
            warp::reply::json(&json!({ "code": 200, "data": results }))
        });

    // === CLEAR GALLERY ===
    let adb_gallery = adb_path.clone();
    let clear_gallery_route = warp::path!("api" / "clear_gallery")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            log::info!("Clearing gallery on devices");
            let result = run_on_serials(&adb_gallery, &body, &[
                "shell", "rm", "-rf", "/sdcard/DCIM/*", "/sdcard/Pictures/*", "/sdcard/Movies/*"
            ]);
            warp::reply::json(&result)
        });

    // === INSTALL/UNINSTALL APK ===
    let adb_install = adb_path.clone();
    let install_apk_route = warp::path!("api" / "install_apk")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            let apk_path = body["path"].as_str().unwrap_or("");
            log::info!("Installing APK: {}", apk_path);
            let result = run_on_serials(&adb_install, &body, &["install", "-r", apk_path]);
            warp::reply::json(&result)
        });

    let adb_uninstall = adb_path.clone();
    let uninstall_apk_route = warp::path!("api" / "uninstall_apk")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            let package = body["package"].as_str().unwrap_or("");
            log::info!("Uninstalling: {}", package);
            let result = run_on_serials(&adb_uninstall, &body, &["uninstall", package]);
            warp::reply::json(&result)
        });

    // === SET TEXT (for input fields) ===
    let adb_text = adb_path.clone();
    let set_text_route = warp::path!("api" / "device" / "set_text")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            let text = body["text"].as_str().unwrap_or("");
            log::info!("Setting text: {}", text);
            let result = run_on_serials(&adb_text, &body, &["shell", "input", "text", text]);
            warp::reply::json(&result)
        });

    // === AUTH/LICENSE (no-op) ===
    let auth_route = warp::path!("api" / "auth").map(|| {
        warp::reply::json(&json!({ "code": 200, "data": { "license": "tikfarm", "plan": "unlimited", "expired": false } }))
    });

    let license_route = warp::path!("api" / "get_license").map(|| {
        let license_data = json!({
            "license_code": "tikfarm-unlimited",
            "plan_name": "TikFarm Pro",
            "leftdays": 9999,
            "is_stripe_active": 1,
            "concurrency_limit": 100,
            "expired_at": "2099-12-31",
            "features": ["whiteLabel", "followPlan", "superMarketing", "aiAgent"]
        });
        warp::reply::json(&json!({ "code": 0, "data": license_data.to_string() }))
    });

    let concurrency_route = warp::path!("api" / "license" / "concurrency_limit").map(|| {
        warp::reply::json(&json!({ "code": 0, "data": 100 }))
    });

    let settings_route = warp::path!("api" / "settings").map(|| {
        warp::reply::json(&json!({ "data": {} }))
    });

    // === ACCOUNT ROUTES (stub) ===
    let account_get_route = warp::path!("api" / "account")
        .and(warp::get())
        .map(|| warp::reply::json(&json!({ "data": [] })));

    let account_post_route = warp::path!("api" / "account")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            log::info!("Add account: {:?}", body);
            warp::reply::json(&json!({ "code": 200, "data": body }))
        });

    let account_count_route = warp::path!("api" / "account" / "count_all")
        .and(warp::get())
        .map(|| warp::reply::json(&json!({ "data": 0 })));

    // === FALLBACK ===
    let fallback = warp::any()
        .and(warp::path::full())
        .and(warp::method())
        .map(|path: warp::path::FullPath, method: warp::http::Method| {
            log::warn!("Unimplemented: {} {}", method, path.as_str());
            warp::reply::json(&json!({ "data": [], "code": 200 }))
        });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
        .allow_headers(vec!["Content-Type", "Authorization", "X-App-Id"]);

    let routes = devices_route
        .or(index_route)
        .or(count_route)
        .or(adb_cmd_route)
        .or(open_tiktok_route)
        .or(stop_tiktok_route)
        .or(clear_data_route)
        .or(grant_tiktok_route)
        .or(clear_gallery_route)
        .or(install_apk_route)
        .or(uninstall_apk_route)
        .or(set_text_route)
        .or(auth_route)
        .or(license_route)
        .or(concurrency_route)
        .or(settings_route)
        .or(account_get_route)
        .or(account_post_route)
        .or(account_count_route)
        .or(fallback)
        .with(cors);

    log::info!("HTTP on port {}, WebSocket on port {}", API_PORT, WS_PORT);
    warp::serve(routes).run(([127, 0, 0, 1], API_PORT)).await;
}
