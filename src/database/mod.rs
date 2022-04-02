// THIS IS ONLY FOR THE USE CASE OF IMPLIMENTING A MYSQL DATABASE
// IF U DONT HAVE ONE, DONT WORRY ABOUT THIS FILE/DATABASE FOLDER :)

pub mod functions;
mod generator;
pub mod models;

use mysql::{prelude::Queryable, Error, Pool, PooledConn};
// use redis::*;

use crate::config::SpaceConfig;

#[allow(unused_imports)]
use self::generator::{database::gen_database, processors::gen_processors};

static mut CONNECTION: Option<Pool> = None;

pub async fn get_database_connection() -> Result<PooledConn, Error> {
    unsafe {
        match &CONNECTION {
            Some(pool) => Ok(pool.get_conn()?),
            None => {
                // let connection_string = format!(
                //     "{}/{}",
                //     SpaceConfig::get_database_connection_string(),
                //     SpaceConfig::get_database_name()
                // );

                let opts = mysql::OptsBuilder::new()
                    .ip_or_hostname(Some(format!("{}", SpaceConfig::get_database_host())))
                    .pass(Some(format!("{}", SpaceConfig::get_database_pass())))
                    .user(Some(format!("{}", SpaceConfig::get_database_user())))
                    .db_name(Some(format!("{}", SpaceConfig::get_database_name())));

                let pool = Pool::new(opts)?;
                let conn = pool.get_conn()?;
                CONNECTION = Some(pool);
                Ok(conn)
            }
        }
    }
}

pub async fn crate_database() -> Result<(), Error> {
    let mut conn = match get_database_connection().await {
        Ok(conn) => conn,
        Err(err) => {
            if let Error::MySqlError(err) = err {
                if err.code == 1049 {
                    // let conn_url = "mysql://root:password@localhost:3307/db_name";

                    let opts = mysql::OptsBuilder::new()
                        .ip_or_hostname(Some(format!("{}", SpaceConfig::get_database_host())))
                        .pass(Some(format!("{}", SpaceConfig::get_database_pass())))
                        .user(Some(format!("{}", SpaceConfig::get_database_user())))
                        .db_name(Some(format!("{}", SpaceConfig::get_database_name())));

                    let pool = Pool::new(opts)?;
                    let mut conn = pool.get_conn()?;
                    conn.query_drop(r"CREATE DATABASE IF NOT EXISTS Space")?;

                    get_database_connection().await?
                } else {
                    return Err(err.into());
                }
            } else {
                return Err(err);
            }
        }
    };

    gen_database(&mut conn).await?;
    gen_processors(&mut conn).await?;

    Ok(())
}