use rustodo::db::{db_init, table_exists};
use rustodo::todo::run;
use rusqlite::{Connection, Result}; 
use std::env;
use std::process;

fn main() -> Result<()> {
    let connection_path = env::var("RUSTODO_DB_PATH").unwrap_or_else(|_| {
        eprintln!("RUSTODO_DB_PATH not set");
        process::exit(1);
    });
    let conn = Connection::open(connection_path)?;
    if !table_exists(&conn)? {
        db_init(&conn)?;
    }
    run(&conn)?;
    Ok(())
}
