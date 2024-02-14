use rusqlite::{Connection, Result};
use clap::{arg, error::{ContextKind, ErrorKind}, Command};
use std::env;
use comfy_table::{Table, Cell, Color};

const TODO_STRING: &'static str = r#"

/$$$$$$$                        /$$                     /$$          
| $$__  $$                      | $$                    | $$          
| $$  \ $$ /$$   /$$  /$$$$$$$ /$$$$$$    /$$$$$$   /$$$$$$$  /$$$$$$ 
| $$$$$$$/| $$  | $$ /$$_____/|_  $$_/   /$$__  $$ /$$__  $$ /$$__  $$
| $$__  $$| $$  | $$|  $$$$$$   | $$    | $$  \ $$| $$  | $$| $$  \ $$
| $$  \ $$| $$  | $$ \____  $$  | $$ /$$| $$  | $$| $$  | $$| $$  | $$
| $$  | $$|  $$$$$$/ /$$$$$$$/  |  $$$$/|  $$$$$$/|  $$$$$$$|  $$$$$$/
|__/  |__/ \______/ |_______/    \___/   \______/  \_______/ \______/ 
"#;

fn cli() -> Command {
    Command::new("todo")
        .about("Todo application")
        .long_about(TODO_STRING)
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
        .subcommand(
            Command::new("complete")
                .about("Complete a todo")
                .alias("done")
                .arg(
                    arg!(<TODO_ID> "The id of the todo to complete")
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
    conn.execute(
        "INSERT INTO Todo (title, owner) VALUES (?1, ?2)",
        (title, "Luke Harwood"),
    )?;
    Ok(())
}

fn remove_todo(conn: &Connection, id: u32) -> Result<()> {

    conn.execute(
        "DELETE FROM Todo WHERE row_id = ?1",
        (id,)
    )?;
    Ok(())
}

fn update_todo(
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

fn config_required(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("
        SELECT name FROM sqlite_master
        WHERE type='table' AND name='Todo'
    ")?;
    let mut rows = stmt.query([])?;
    Ok(rows.next()?.is_none())
}

// const HOME_PATH: &'static str = "/Users/lukeharwood/.config/todo/todo.db";

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
    let connection_path = env::var("RUSTODO_DB_PATH").unwrap_or_else(|_| panic!("RUSTODO_DB_PATH not set"));
    let conn = Connection::open(connection_path)?;
    if config_required(&conn)? {
        println!("Initializing database.");
        db_init(&conn)?;
    }
    match cli().get_matches().subcommand() {
        None => {
            // print todos
            let todos = get_todos(&conn)?;

            let mut table = Table::new();
            table.set_header(vec!["ID", "Todo", "Completed", "Owner", "Created At", "Updated At"]);

            for todo in todos {
                let row: Vec<String> = vec![
                    todo.row_id.to_string(),
                    todo.title,
                    if todo.completed {"yes".to_string() } else { "nope".to_string() },
                    todo.owner,
                    todo.updated_at,
                    todo.created_at,
                ];
                table.add_row(
                    row.iter().map(|c| Cell::new(c).bg(
                        if todo.completed {
                            Color::Green
                        } else {
                            Color::Red
                        }
                    ).fg(
                        Color::White
                    ))
                );            
                println!("{table}");
            }
        },
        Some(("add", x)) => {
            if let Some(todo) = x.get_one::<String>("TODO") {
                add_todo(&conn, todo.to_owned())?;
                println!("Added todo: '{todo}'!", todo=todo);
            }
        },
        Some(("remove", x)) => {
            if let Some(id) = x.get_one::<String>("TODO_ID") {
                if let Ok(id) = id.parse::<u32>() {
                    remove_todo(&conn, id)?;
                    println!("Removed todo: {id}!", id=id);
                } else {
                    let mut err = clap::Error::new(ErrorKind::InvalidValue);
                    err.insert(ContextKind::InvalidValue, clap::error::ContextValue::String("TODO_ID".to_string()));
                    let _ = err.print();
                }
            }
        },
        Some(("complete", x)) => {
            if let Some(id) = x.get_one::<String>("TODO_ID") {
                if let Ok(id) = id.parse::<u32>() {
                    let todos = get_todos(&conn)?;
                    let todo = todos.iter().find(|t| t.row_id == id);
                    if let Some(todo) = todo {
                        let todo = Todo::new(
                            todo.row_id,
                            todo.title.clone(),
                            true,
                            todo.owner.clone(),
                            todo.updated_at.clone(),
                            todo.created_at.clone(),
                        );
                        update_todo(&conn, todo)?;
                        println!("Completed todo: {id}!", id=id);
                    } else {
                        println!("No todo found with id: {id}!", id=id);
                    }
                } else {
                    let mut err = clap::Error::new(ErrorKind::InvalidValue);
                    err.insert(ContextKind::InvalidValue, clap::error::ContextValue::String("TODO_ID".to_string()));
                    let _ = err.print();
                }
            }
        },
        Some(("update", x)) => println!("update - {x:?}"),
        Some(("reset", x)) => {
            if let Some(force) = x.get_one::<bool>("force") {
                if *force {
                    db_init(&conn)?;
                    println!("Resetting database!");
                } else {
                    println!("You must use the --force (-f) flag to reset the database");
                }
            }
        },
        x @ _ => println!("{:?}", x),
    }
    Ok(())
}
