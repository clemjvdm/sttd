#![allow(dead_code,unused_variables)]
mod structs;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use serde_json;
use structs::{ ToDoList, ToDoItem};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Create { name: String},
    Remove { index: usize},
    Select {
        index: usize,
        #[command(subcommand)]
        command: SelectCommands,
    },
}

#[derive(Subcommand)]
enum SelectCommands {
    List,
    Add { name: String},
    Remove { index: usize},
}

const DATA_PATH: &str = "sttd_data";

fn save_list(list: Vec<ToDoList>) -> File {
    let mut file = std::fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(DATA_PATH)
                            .unwrap();
    let json = serde_json::to_string(&list).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    return file;
}

fn main() {

    // open lists file
    let file_result = std::fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(DATA_PATH);

    // if file doesn't exist create new file with empty Vec<ToDoList>
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => save_list(Vec::new()),
            other_error => {
                panic!("Problem opening file: {}", error);
            }
        }
    };

    // read lists file
    let json = std::fs::read_to_string(DATA_PATH).unwrap();
    let mut lists: Vec<ToDoList> = serde_json::from_str(&json).unwrap();

    // cli
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            if lists.is_empty() { println!("No lists, use 'create' to create a new list.") }
            for (index, list) in lists.iter().enumerate() {
                println!("{}\t{}", index+1, list.name);
            }
        }

        Commands::Create { name } => {
            let list = ToDoList::new(name.to_string());
            lists.push(list);
            save_list(lists);
            println!("Created list {}", name);
        }

        Commands::Remove { index } => {
            let list = lists.get(*index-1).expect("Index out of bounds.");
            println!("Removing list {}", list.name);
            lists.remove(*index-1);
            save_list(lists);
        }

        Commands::Select { index, command } => {
            let list;
            match lists.get_mut(*index) {
                Some(s) => {list = s;},
                None => {
                    println!("Index out of bounds");
                    return;
                }
            };

            match command {
                SelectCommands::List => {
                    for (index, todo) in list.todos.iter().enumerate() {
                        println!("{}\t{}", index+1, todo.name);
                    }
                }

                SelectCommands::Add { name } => {
                    list.add_todo(ToDoItem::new(name.to_string()));
                    save_list(lists);
                }

                SelectCommands::Remove { index } => {
                    list.del_todo(*index-1);
                    save_list(lists);
                }
            }
        }
    }
}
