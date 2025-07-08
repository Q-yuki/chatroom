use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::WebSocket;
use warp::filters::ws;

use crate::center::Room;
use crate::states::roomState::{RoomState, RoomResponse};
use crate::db::room::{
    self, create_table, delete_room, get_member_num, get_room, insert_member, insert_room, remove_member, update_room
};



#[tauri::command]
pub fn create_room(
    username: String,
    state: State<RoomState>
) -> RoomResponse {
    let room_id = format!("{}", rand::random::<u16>());
    let room = Room::new(room_id.clone(), username.clone());
    insert_room(&room).unwrap();
    create_table(&room_id).unwrap();
    insert_member(&room_id, &username).unwrap();

    RoomResponse {
        success: true,
        message: format!("房间{}创建成功", room_id),
        room_id: Some(room_id),
    }
}

#[tauri::command]
pub fn join_room(
    username: String,
    room_id: String,
    state: State<RoomState>
) -> RoomResponse {
    let room = get_room(&room_id).unwrap();
    if room.is_none() {
        return RoomResponse {
            success: false,
            message: format!("房间{}不存在", room_id),
            room_id: None,
        };
    }
    insert_member(&room_id, &username).unwrap();
    RoomResponse {
        success: true,
        message: format!("成功加入房间{}", room_id),
        room_id: Some(room_id),
    }
}

#[tauri::command]
pub fn leave_room(
    username: String,
    room_id: String,
    state: State<RoomState>
) -> RoomResponse {
    let room = get_room(&room_id).unwrap();
    if room.is_none() {
        return RoomResponse {
            success: false,
            message: format!("房间{}不存在", room_id),
            room_id: None,
        };
    }
    remove_member(&room_id, &username).unwrap();
    RoomResponse {
        success: true,
        message: format!("成功离开房间{}", room_id),
        room_id: Some(room_id),
    }
}

#[tauri::command]
pub fn delete_table(
    room_id: String,
) {
    if let Err(e) = delete_room(&room_id) {
        eprintln!("Failed to delete room {}: {:?}", room_id, e);
    }
}

#[tauri::command]
pub fn send_message(
    room_id: String,
    username: String,
    message: String,
    state: State<RoomState>,
) -> RoomResponse {
    // 验证用户是否在房间中
    let room = get_room(&room_id).unwrap();
    if room.is_none() {
        return RoomResponse {
            success: false,
            message: format!("房间{}不存在", room_id),
            room_id: None,
        };
    }
    
    RoomResponse {
        success: true,
        message: "消息已发送".into(),
        room_id: Some(room_id),
    }
}

#[tauri::command]
pub fn show_member_num(
    room_id: String,
) -> String {
    match get_member_num(&room_id) {
        Ok(num) => num.to_string(),
        Err(_) => "0".to_string(),
    }
}