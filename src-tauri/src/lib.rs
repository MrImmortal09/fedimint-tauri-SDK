// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod rpc;

use rpc::{AppState, initialize_rpc_handler, handle_rpc_request, get_app_status};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .setup(|_app| {
            // You can initialize app-wide state here if needed
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Fedimint RPC commands
            initialize_rpc_handler,
            handle_rpc_request,
            get_app_status

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
