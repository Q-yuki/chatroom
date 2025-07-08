use mysql::*;
use mysql::prelude::*;
use crate::center::Room;

const DB_URL: &str = "mysql://root:123456@127.0.0.1:3306/test";

fn get_connection() -> Result<PooledConn, mysql::Error> {
    let pool = Pool::new(DB_URL).unwrap();
    pool.get_conn()
}

pub fn get_owner(room:&Room) -> Result<String, mysql::Error> {
    let mut conn = get_connection()?;
    let result: Option<String> = conn.exec_first(
        r"SELECT owner FROM rooms WHERE id = :id",
        params! {
            "id" => room.get_id(),
        }
    )?;
    Ok(result.unwrap_or_default())
}

pub fn create_table(id:&str) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let table_name = format!("rooms_{}", id);
    conn.query_drop(
        format!("CREATE TABLE IF NOT EXISTS {} (
            members VARCHAR(255) NOT NULL
        )", table_name)
    )?;
    Ok(())
}

pub fn insert_room(room: &Room) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.exec_drop(
        r"INSERT INTO rooms (id, owner) VALUES (:id, :owner)",
        params! {
            "id" => room.get_id(),
            "owner" => room.get_owner(),
        }
    )?;
    Ok(())
}

pub fn get_room(id: &str) -> Result<Option<(String, String)>, mysql::Error> {
    let mut conn = get_connection()?;
    let result: Option<(String, String)> = conn.exec_first(
        r"SELECT id, owner FROM rooms WHERE id = :id",
        params! {
            "id" => id,
        }
    )?;
    Ok(result)
}

pub fn delete_room(id: &str) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.exec_drop(
        r"DELETE FROM rooms WHERE id = :id",
        params! {
            "id" => id,
        }
    )?;
    let table_name = format!("rooms_{}", id);
    conn.exec_drop(
        format!("DROP TABLE IF EXISTS {}", table_name),
        Params::Empty,
    )?;
    Ok(())
}

pub fn update_room(id: &str, owner: &str) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.exec_drop(
        r"UPDATE rooms SET owner = :owner WHERE id = :id",
        params! {
            "id" => id,
            "owner" => owner,
        }
    )?;
    Ok(())
}

pub fn insert_member(room_id: &str, member: &str) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let table_name = format!("rooms_{}", room_id);
    conn.exec_drop(
        format!("INSERT INTO {} (members) VALUES (:member)", table_name),
        params! {
            "member" => member,
        }
    )?;
    Ok(())
}

pub fn remove_member(room_id: &str, member: &str) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let table_name = format!("rooms_{}", room_id);
    conn.exec_drop(
        format!("DELETE FROM {} WHERE members = :member", table_name),
        params! {
            "member" => member,
        }
    )?;
    Ok(())
}

pub fn get_member_num(room_id: &str) -> Result<u64, mysql::Error> {
    let mut conn = get_connection()?;
    let table_name = format!("rooms_{}", room_id);
    let result: Option<u64> = conn.exec_first(
        format!("SELECT COUNT(*) FROM {}", table_name),
        Params::Empty,
    )?;
    Ok(result.unwrap_or(0))
}