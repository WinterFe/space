use mysql::{params, prelude::Queryable};
use serenity::{
    framework::standard::{CommandResult},
};

use crate::database::{
    get_database_connection
};

pub async fn register_user(uid: i64, uname: String, bname: String) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.exec_drop(
        r"
        INSERT IGNORE INTO users (discord_id, name)
        VALUES (:discord_id, :name)
    ",
        params! {
            "discord_id" => uid,
            "name" => uname
        },
    )?;

    conn.exec_drop(
        r"
        INSERT IGNORE INTO bans (discord_id, name)
        VALUES (:discord_id, :name)
    ",
        params! {
            "discord_id" => uid,
            "name" => bname
        },
    )?;

    Ok(())
}

pub async fn set_non_pingable(uid: i64, pingable: String) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.exec_drop(
        r"
        UPDATE users
        SET pingable = :pingable
        WHERE discord_id = :discord_id
    ",
        params! {
            "pingable" => pingable,
            "discord_id" => uid
        },
    )?;

    let rows = conn.affected_rows();

    if rows == 1 {
        println!("[DB] User pingability updated ({:?})", uid);
        Ok(())
    } else {
        Err("[DB] User not registered".into())
    }
}