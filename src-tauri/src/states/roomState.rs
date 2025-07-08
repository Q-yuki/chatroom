use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct RoomState {
    pub rooms: Mutex<HashMap<String, Vec<String>>>, // room_id -> members
}

#[derive(Serialize, Deserialize)]
pub struct RoomResponse {
    pub success: bool,
    pub message: String,
    pub room_id: Option<String>,
}