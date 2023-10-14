use crate::configuration::model::Database;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn connect_database(config: Database) -> Result<PgConnection, ConnectionError> {
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        config.username, config.password, config.host, config.database,
    );

    let pg_connection = match PgConnection::establish(&database_url) {
        Ok(connection) => connection,
        Err(err) => return Err(err),
    };

    Ok(pg_connection)
}
