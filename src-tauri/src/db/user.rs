use mysql::*;
use mysql::prelude::*;
use crate::center::User;

const DB_URL: &str = "mysql://root:123456@127.0.0.1:3306/test"; // 数据库连接字符串

fn get_connection() -> Result<PooledConn, mysql::Error> {
    let pool = Pool::new(DB_URL).unwrap();
    pool.get_conn()
}

fn create_table() -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            islogin VARCHAR(255) NOT NULL 
        )"
    )?;
    Ok(())
}

pub fn insert_user(user: &User) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.exec_drop(
        r"INSERT INTO users (username, password,islogin) VALUES (:username, :password,:islogin)",
        params! {
            "username" => user.get_username(),
            "password" => user.get_password(),
            "islogin" => "",
        }
    )?;
    Ok(())
}

pub fn get_user(username: &str) -> Result<Option<User>, mysql::Error> {
    let mut conn = get_connection()?;
    let result: Option<(u32, String, String)> = conn.exec_first(
        r"SELECT id, username, password FROM users WHERE username = :username",
        params! {
            "username" => username,
        }
    )?;
    Ok(result.map(|(id, username, password)| User::new(username, password)))
}

pub fn userlogin(user: &mut User) -> Result<bool, mysql::Error> {
    let mut conn = get_connection()?;
    let result: Option<(u32, String, String, String)> = conn.exec_first(
        r"SELECT id, username, password, islogin FROM users WHERE username = :username AND password = :password",
        params! {
            "username" => user.get_username(),
            "password" => user.get_password(),
        }
    )?;
    // 判断 islogin 字段是否为 "true"
    if let Some((_id, _username, _password, islogin)) = &result {
        if islogin == "true" {
            user.set_islogin("true".to_string());
            return Ok(false);
        }
        let islogin:Option<String> = conn.exec_first(
            r"UPDATE users SET islogin = 'true' WHERE username = :username",
            params! {
                "username" => user.get_username(),
            }
        )?;
        return Ok(true);
    } else {
        //当前用户不存在
        return Ok(false);
    }
}

pub fn userlogout(user: &User) -> Result<bool, mysql::Error> {
    let mut conn = get_connection()?;
    let result = conn.exec_drop(
        r"UPDATE users SET islogin = 'no' WHERE username = :username",
        params! {
            "username" => user.get_username(),
        }
    )?;
    Ok(true)
}

pub fn update_user(user: &User) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.exec_drop(
        r"UPDATE users SET password = :password WHERE username = :username",
        params! {
            "username" => user.get_username(),
            "password" => user.get_password(),
        }
    )?;
    Ok(())
}

pub fn delete_user(user: &User) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.exec_drop(
        r"DELETE FROM users WHERE username = :username",
        params! {
            "username" => user.get_username(),
        }
    )?;
    Ok(())
}