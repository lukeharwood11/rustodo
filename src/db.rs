use rusqlite::{Connection, Result};
use crate::todo::Todo;

pub fn add_todo(conn: &Connection, title: String) -> Result<()> {
    conn.execute(
        "INSERT INTO Todo (title, owner) VALUES (?1, ?2)",
        (title, "Luke Harwood"),
    )?;
    Ok(())
}

pub fn remove_todo(conn: &Connection, id: u32) -> Result<()> {

    conn.execute(
        "DELETE FROM Todo WHERE row_id = ?1",
        (id,)
    )?;
    Ok(())
}

pub fn update_todo(
    conn: &Connection,
    todo: Todo,
) -> Result<()> {
    conn.execute(
        "UPDATE Todo SET title = ?1, completed = ?2, updated_at = current_timestamp WHERE row_id = ?3",
        (todo.title, todo.completed, todo.row_id),
    )?;
    Ok(())
}

// TODO: implement config for owner/metadata

pub fn get_todos(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn
        .prepare("SELECT row_id, title, completed, owner, updated_at, created_at FROM Todo")?;

    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ))
    })?;

    Ok(
        todo_iter.filter(|t| t.is_ok()).map(|t| t.unwrap()).collect()
    )
}


// const HOME_PATH: &'static str = "/Users/lukeharwood/.config/todo/todo.db";
pub fn table_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("
        SELECT name FROM sqlite_master
        WHERE type='table' AND name='Todo'
    ")?;
    let mut rows = stmt.query([])?;
    Ok(rows.next()?.is_some())
}

pub fn db_init(conn: &Connection) -> Result<()> {
    // create tables
    conn.execute("DROP TABLE IF EXISTS Todo", ())?;
    conn.execute("CREATE TABLE Todo (
        row_id INTEGER PRIMARY KEY ASC,
        title VARCHAR(100),
        completed BOOL DEFAULT FALSE,
        owner VARCHAR(100),
        updated_at DATETIME DEFAULT current_timestamp,
        created_at DATETIME DEFAULT current_timestamp)",
        (),
    )?;
    Ok(())
}