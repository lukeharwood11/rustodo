use clap::{arg, Command};

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

pub fn cli() -> Command {
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