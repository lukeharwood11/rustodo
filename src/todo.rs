use crate::commands::cli;
use crate::db::{add_todo, db_init, get_todos, remove_todo, update_todo};
use clap;
use clap::error::{ContextKind, ErrorKind};
use rusqlite::{Connection, Result};
use comfy_table::{Table, Cell, Color};
pub const TODO_STRING: &'static str = r#"

/$$$$$$$                        /$$                     /$$          
| $$__  $$                      | $$                    | $$          
| $$  \ $$ /$$   /$$  /$$$$$$$ /$$$$$$    /$$$$$$   /$$$$$$$  /$$$$$$ 
| $$$$$$$/| $$  | $$ /$$_____/|_  $$_/   /$$__  $$ /$$__  $$ /$$__  $$
| $$__  $$| $$  | $$|  $$$$$$   | $$    | $$  \ $$| $$  | $$| $$  \ $$
| $$  \ $$| $$  | $$ \____  $$  | $$ /$$| $$  | $$| $$  | $$| $$  | $$
| $$  | $$|  $$$$$$/ /$$$$$$$/  |  $$$$/|  $$$$$$/|  $$$$$$$|  $$$$$$/
|__/  |__/ \______/ |_______/    \___/   \______/  \_______/ \______/ 
"#;


#[derive(Debug)]
pub struct Todo {
    pub row_id: u32,
    pub title: String,
    pub completed: bool,
    pub owner: String,
    pub updated_at: String,
    pub created_at: String,
}

impl Todo {
    pub fn new(
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

pub fn run(conn: &Connection) -> Result<()> {
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
            }
            println!("{table}");
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