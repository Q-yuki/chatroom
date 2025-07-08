use serde::{Deserialize, Serialize};
use tauri::State;
use crate::states::authState::{AppState, AuthResponse};
use crate::center::User;
use crate::db::user::{
    insert_user,
    get_user,
    update_user,
    delete_user,
    userlogin,
    userlogout,
};

#[tauri::command]
pub fn login(
    username: String,
    password: String,
    state: State<AppState>
) -> AuthResponse {
    let mut user = User::new(username.clone(), password.clone());
    if userlogin(&mut user).unwrap_or(false) {
        return AuthResponse {
            success: true,
            message: "登录成功".into(),
            username: Some(username),
            token: Some("user_token".into()),
        };
    } else if user.get_islogin() == "true"{
        return AuthResponse {
            success: false,
            message: "当前用户已登录".into(),
            username: None,
            token: None,
        };
    } else {
        return AuthResponse {
            success:false,
            message:"账号或密码错误".into(),
            username:None,
            token:None,
        }
    }
}

#[tauri::command]
pub fn register(
    username: String,
    password: String,
    state: State<AppState>
) -> AuthResponse {
    let user= User::new(username.clone(), password.clone());
    if get_user(&user.get_username()).unwrap().is_some() {
        AuthResponse {
            success: false,
            message: "用户名已存在".into(),
            username: None,
            token: None,
        }
    } else {
        let user = User::new(username.clone(), password.clone());
        // Insert into database
        insert_user(&user).unwrap();
        AuthResponse {
            success: true,
            message: "注册成功".into(),
            username: Some(username),
            token: Some("user_token".into()),
        }
    }
}

#[tauri::command]
pub fn logout(
    username: String,
    state: State<AppState>
) -> AuthResponse {
    // If password is not needed for logout, use an empty string or a default value
    let user = User::new(username.clone(), "".to_string());
    if userlogout(&user).unwrap() {
        AuthResponse {
            success: true,
            message: "成功登出".into(),
            username: Some(username),
            token: Some("user_token".into()),
        }
    } else {
        AuthResponse {
            success: false,
            message: "登出失败".into(),
            username: Some(username),
            token: Some("user_token".into()),
        }
    }
}