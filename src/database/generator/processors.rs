use mysql::{Error, PooledConn};

pub async fn gen_processors(_conn: &mut PooledConn) -> Result<(), Error> {
    Ok(())
}