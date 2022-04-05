use mysql::{prelude::Queryable, Error, PooledConn};

use crate::config::SpaceConfig;

pub async fn gen_database(conn: &mut PooledConn) -> Result<(), Error> {
    conn.query_drop(format!(
        r"
        CREATE TABLE IF NOT EXISTS public.guilds (
            discord_id BIGINT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            prefix VARCHAR(15) NOT NULL DEFAULT '{}',
            guild_type INT NOT NULL DEFAULT 0
        )
    ",
        SpaceConfig::get_default_prefix()
    ))?;

    conn.query_drop(
        r"
        CREATE TABLE IF NOT EXISTS users (
            discord_id BIGINT PRIMARY KEY,
            name VARCHAR(255) NOT NULL
        )
    ",
    )?;

    conn.query_drop(
        r"
        CREATE TABLE IF NOT EXISTS fans (
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL
        )
    ",
    )?;

    conn.query_drop(
        r"
        create sequence status_seq
        start 1
        increment 1
        NO MAXVALUE
        CACHE 1;
    ",
    )?;

    conn.query_drop(
        r"
        CREATE TABLE IF NOT EXISTS status (
            id INT UNIQUE NOT NULL,
            status VARCHAR(100) NOT NULL
        )
    ",
    )?;

    conn.query_drop(
        r"
        create sequence custom_reactions_seq
        start 1
        increment 1
        NO MAXVALUE
        CACHE 1;
    ",
    )?;

    conn.query_drop(
        r"
        CREATE TABLE IF NOT EXISTS custom_reactions (
            id INT UNIQUE NOT NULL,
            question TEXT NOT NULL,
            reply TEXT NOT NULL,
            cr_type INT NOT NULL DEFAULT 0,
            guild_id BIGINT NOT NULL,
            FOREIGN KEY (guild_id) REFERENCES guilds(discord_id)
        )
    ",
    )?;

    Ok(())
}
