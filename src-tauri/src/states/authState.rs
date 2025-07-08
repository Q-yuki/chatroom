use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub users: Mutex<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub username: Option<String>,
    pub token: Option<String>,
}