use mysql::{params, prelude::Queryable};
use serenity::{
    framework::standard::{CommandResult},
};

use crate::database::{
    get_database_connection
};

pub async fn set_banned(uid: i64, banned: &str, banned_at: &str) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.exec_drop(
        r"
        UPDATE bans
        SET banned = :banned,
        banned_at = :banned_at
        WHERE discord_id = :discord_id
    ",
        params! {
            "banned" => banned,
            "banned_at" => banned_at,
            "discord_id" => uid
        },
    )?;

    let rows = conn.affected_rows();

    if rows == 1 {
        println!("[DB] User temp banned ({:?})", uid);
        Ok(())
    } else {
        Err("[DB] User not registered".into())
    }
}

pub async fn create_ban_event(uid: i64, event: String) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.query_drop(format!("CREATE EVENT IF NOT EXISTS {} ON SCHEDULE AT CURRENT_TIMESTAMP + INTERVAL 1 WEEK DO UPDATE bans SET banned = 'false', banned_at = NULL WHERE discord_id = {};", event, uid))?;

    Ok(())
}