use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ToDoItem {
    pub name: String,
    pub todos: Vec<ToDoItem>,
    pub done: bool,
}

impl ToDoItem {
    pub fn new(name: String) -> Self {
        ToDoItem {name, todos: Vec::new(), done: false}        
    }    
   
    fn add_child (&mut self, todo: ToDoItem) {
        self.todos.push(todo);
    }
}


#[derive(Serialize, Deserialize)]
pub struct ToDoList {
    pub name: String,
    pub todos: Vec<ToDoItem>,
}


impl ToDoList {
    pub fn new(name: String) -> Self {
        let todos = Vec::new(); 
        ToDoList {name, todos}
    }

    pub fn from_json(path: String) -> Self {
        let json = std::fs::read_to_string(path).unwrap();
        let obj: ToDoList = serde_json::from_str(&json).unwrap();
        return obj;
    }

    pub fn add_todo(&mut self, todo: ToDoItem) {
        self.todos.push(todo);
    }

    pub fn del_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }
}
