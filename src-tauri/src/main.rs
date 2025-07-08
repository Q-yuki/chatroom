#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod states;
mod db;
mod center;
mod server;

use tauri::{Manager, generate_handler,Emitter};
use commands::{auth, room};
use states::{authState, roomState};
use std::sync::Mutex;
use server::se;

fn run() {
        tauri::Builder::default()
        .manage(authState::AppState {
            users: std::sync::Mutex::new(std::collections::HashMap::new()),
        })
        .manage(roomState::RoomState {
            rooms: std::sync::Mutex::new(std::collections::HashMap::new()),
        })
        .invoke_handler(generate_handler![
            auth::login,
            auth::logout,
            auth::register,
            room::create_room,
            room::join_room,
            room::delete_table,
            room::send_message,
            room::leave_room,
            room::show_member_num,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();
            tokio::spawn(async {
                se::run_websocket_server().await;
            });
            Ok(())
       })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tokio::main]
async fn main() {
    run();
}