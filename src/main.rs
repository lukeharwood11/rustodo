use rusqlite::{Connection, Result};
use clap::{arg, Command};

fn cli() -> Command {
    Command::new("todo")
        .about("Todo application")
        .subcommand(
            Command::new("reset")
                .about("Reset the current state of the todo application")
                .arg(arg!(-f --force "Confirm erasing all todos"))
        )
        .subcommand(
            Command::new("add")
                .about("Add a new todo")
                .alias("new")
                .arg(
                    arg!(<TODO> "Add a new todo")
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a todo")
                .alias("rm")
                .arg(
                    arg!(<TODO_ID> ... "Remove a todo")
                )
        )
        .subcommand(
            Command::new("update")
                .about("Update a todo")
                .alias("up")
                .arg(
                    arg!(<TODO_ID> "Update a todo")
                )
        )
}
#[derive(Debug)]
struct Todo {
    row_id: u32,
    title: String,
    completed: bool,
    owner: String,
    updated_at: String,
    created_at: String,
}

impl Todo {
    fn new(
        row_id: u32,
        title: String,
        completed: bool,
        owner: String,
        updated_at: String,
        created_at: String,
    ) -> Todo {
        Todo {
            row_id,
            title,
            completed,
            owner,
            updated_at,
            created_at,
        }
    }
}

fn add_todo(conn: &Connection, title: String) -> Result<()> {
    conn
        .execute(
            "INSERT INTO Todo (title"
        )
}

fn get_todos(conn: &Connection) -> Result<Vec<Todo>> {
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

const HOME_PATH: &'static str = "/Users/lukeharwood/.config/todo/todo.db";

fn db_init(conn: &Connection) -> Result<()> {
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

fn main() -> Result<()> {
    let conn = Connection::open(HOME_PATH)?;
    db_init(&conn)?;
    match cli().get_matches().subcommand() {
        None => println!("No matches..."),
        Some(("add", x)) => {
            if let Some(todo) = x.get_one::<String>("TODO") {
                println!("Adding {todo}")
            }
        },
        Some(("remove", x)) => {

        },
        Some(("update", x)) => println!("update - {x:?}"),
        x @ _ => println!("{:?}", x),
    }
    Ok(())
}
