#[macro_use]
extern crate aerospike;

use std::{time::Duration, collections::{HashMap, hash_map::RandomState}};
use aerospike::{Bins, Client, ClientPolicy, QueryPolicy, Statement, WritePolicy, Record, Value};


#[derive(Debug)]
struct Todo {
    id: i64,
    task: String,
    is_complete: bool
}

impl Todo {
    fn get_key_from_record(key: aerospike::Key) -> i64 {
        match key.user_key {
            Some(aerospike::Value::Int(k)) => k,
            _ => 0,
        }
    }

    fn get_task_from_record(bins: &HashMap<String, Value, RandomState>) -> String {
        match bins.get("task") {
            Some(aerospike::Value::String(t)) => t.to_string(),
            _ => String::from("WARNING! Received unexpected string from database!")
        }
    }

    fn get_completion_from_record(bins: &HashMap<String, Value, RandomState>) -> bool {
        match bins.get("is_complete") {
            Some(aerospike::Value::Bool(c)) => *c,
            _ => false,
        }
    }

    fn parse_todo(rec: Record) -> Todo {
        let key = Self::get_key_from_record(rec.key.unwrap());
        let task = Self::get_task_from_record(&rec.bins);
        let is_complete = Self::get_completion_from_record(&rec.bins);

        let todo = Todo {
            id: key,
            task,
            is_complete
        };
        
        return todo
    }
}

struct Todos {
    client: aerospike::Client,
    todos: Vec<Todo>,
}

impl Todos {
    fn new(host: &str) -> Todos {
        //let todos: Vec<Todo> = vec![];
        let mut cpolicy = ClientPolicy::default();
        cpolicy.timeout = Some(Duration::new(1, 0));

        let client = Client::new(&cpolicy, &host)
            .expect("Failed to connect to cluster");
        Todos {
            client: client,
            todos: vec![],
        }
    }
    fn get_all_todos(&mut self) {
        let qpolicy = QueryPolicy::new();
        let stmt = Statement::new("test", "todos", Bins::All);

        match self.client.query(&qpolicy, stmt) {
            Ok(records) => {
                records.for_each(|rec| match rec {
                    Ok(r) => { 
                        let todo = Todo::parse_todo(r);
                        self.todos.push(todo);
                    },
                    Err(e) => println!("Error: {} ", e),
                });
            },
            _ => {
                println!("Error in query");
            }
        }
    }
    fn print_all_todos(&self) {
        self.todos
            .iter()
            .for_each(|td| println!("ID: {}, Task: {}, Completed: {}", td.id, td.task, td.is_complete))
    }
    fn create_todo(&mut self, task: &str, id: i64) {
        if let Some(_) = self.todos.iter_mut().find(|todo| todo.id == id) {
            println!("WARNING! ID {} already exists!", id);
            return
        }
        let mut wpolicy = WritePolicy::new(0,aerospike::Expiration::Seconds(0));
        wpolicy.send_key = true;
        let key = as_key!("test", "todos", &id);
        let bins = [
            as_bin!("task", task),
            as_bin!("is_complete", false)
        ];

        match self.client.put(&wpolicy, &key, &bins) {
            Ok(()) => {
                self.todos.push(Todo{id, task:task.to_string(), is_complete: false}); 
                println!("Wrote record ID {} successfully", &id);
            },
            Err(e) => println!("Failed to write record {}", e)
        }
    }

    fn delete_todo(&mut self, id: i64) {
        let wpolicy = WritePolicy::default();
        let key = as_key!("test", "todos", &id);
        match self.client.delete(&wpolicy, &key) {
            Ok(true) => {
                self.todos.retain(|t| t.id != id);
                println!("Deleted record ID {}", &id);
            },
            Ok(false) => println!("Key {} didn't exist", &id),
            Err(e) => println!("Error in deleting key: {}", e)
        }
    }

    fn complete_todo(&mut self, id: i64) {
        let wpolicy = WritePolicy::default();
        let key = as_key!("test", "todos", &id);
        let bins = [as_bin!("is_complete", true)];

        match self.client.put(&wpolicy, &key, &bins) {
            Ok(()) => { 
                if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
                    todo.is_complete = true;
                    println!("Marked record ID {} as completed successfully", id);
                }
            },
            Err(e) => println!("Failed to mark record {} as completed", e)
        }
    }

}

fn main() {
    let mut client = Todos::new("172.17.0.3:3101");
    client.get_all_todos();
    client.create_todo("learn rust", 1);
    client.create_todo("make a todo app", 2);
    client.create_todo("test the todo app", 3);
    client.create_todo("profit", 4);
    client.print_all_todos();
    client.delete_todo(1);
    client.print_all_todos();
    client.complete_todo(2);
    client.create_todo("my new task", 5);
    client.print_all_todos();
}

/* === RESULTS === 
Wrote record ID 1 successfully
WARNING! ID 2 already exists!
WARNING! ID 3 already exists!
WARNING! ID 4 already exists!
ID: 3, Task: test the todo app, Completed: false
ID: 2, Task: make a todo app, Completed: true
ID: 5, Task: my new task, Completed: false
ID: 4, Task: profit, Completed: false
ID: 1, Task: learn rust, Completed: false
Deleted record ID 1
ID: 3, Task: test the todo app, Completed: false
ID: 2, Task: make a todo app, Completed: true
ID: 5, Task: my new task, Completed: false
ID: 4, Task: profit, Completed: false
Marked record ID 2 as completed successfully
WARNING! ID 5 already exists!
ID: 3, Task: test the todo app, Completed: false
ID: 2, Task: make a todo app, Completed: true
ID: 5, Task: my new task, Completed: false
ID: 4, Task: profit, Completed: false
*/
