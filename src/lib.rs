use std::{io::Write, error::Error};
use std::{process, sync::atomic::{AtomicU64, self}};
use regex::Regex;
use mysql::*;
use mysql::prelude::*;

struct Task {
    id: Option<u64>,
    task: String,
    status: bool,
}

static UNIQUE_ID: AtomicU64  = AtomicU64::new(1);

impl Task {
    fn new(task: String) -> Task {
        Task{id: None, task, status:false}
    }
}

pub struct Todo {
    pub id: u64,
    pub name: String,
}

impl Todo {
    pub fn new(name: &str, conn: & mut PooledConn) -> Todo {

        conn.exec_drop(r"INSERT INTO Todo (name) VALUES (:name)",
             params! {
                "name" => name,
            }).expect("Could not execute query !");

        let id = conn.last_insert_id();

        Todo {id, name: String::from(name)}
    }

    fn exists(& self, task_id: u64, conn: &mut PooledConn) -> bool {
        let result: Option<u64> = conn
            .exec_first(
                "SELECT COUNT(*) FROM Tasks WHERE task_id = :task_id and todo_id = :todo_id",
                params! {
                    "task_id" => task_id,
                    "todo_id" => self.id,
                },
            )
            .expect("Could not execute query");

        match result {
            Some(count) => count > 0,
            None => false,
        }
    }

    fn add_task(& mut self, task: Task, conn: & mut PooledConn) {

        conn.exec_drop(r"insert into Tasks (todo_id, description, status) 
            values (:todo_id, :description, :status)", 
                params! {
                    "todo_id" => self.id,
                    "description" => task.task,
                    "status" => {
                        if task.status {
                            1
                        } else {
                            0
                        }
                    },
                }).expect("Could not execute query !");
    }

    fn delete_task(& mut self, id: u64, conn: & mut PooledConn) -> Result<(), &'static str> {
        if !self.exists(id, conn) {
            return Err("No task with the given id");
        }

        conn.exec_drop("delete from Tasks
            where todo_id = :todo_id and task_id = :task_id", params! {
                "todo_id" => self.id,
                "task_id" => id,
            }).expect("Could not execute query");

        Ok(())
    }

    fn update_task(& mut self, id: u64, task: &str, conn: & mut PooledConn ) -> Result<(), &'static str> {
        if !self.exists(id, conn) {
            return Err("No task with the given id");
        }

        conn.exec_drop("update Tasks 
            set description = :task
            where task_id = :task_id and todo_id = :todo_id",params! {
                "task" => task,
                "task_id" => id,
                "todo_id" => self.id,
            }).expect("Could not execute query");

        return Ok(());
    }

    fn update_task_status(& mut self, id: u64, conn: & mut PooledConn) -> Result<(), &'static str> {
        if !self.exists(id, conn) {
            return Err("No task with the given id");
        }

        conn.exec_drop("update Tasks 
            set status = 1 
            where task_id = :task_id and todo_id = :todo_id",params! {
                "task_id" => id,
                "todo_id" => self.id,
            }).expect("Could not execute query");

        return Ok(());
    }

    fn get_tasks(& self, conn: & mut PooledConn) -> Vec<Task> {
        let tasks: Vec<Task> = conn.exec_map("select task_id, description, status 
                   from Tasks
                   where todo_id = :todo_id", params! {
                       "todo_id" => self.id,
                   },
                   |(id, description, status): (u64, String, u64)| {
                       Task {
                           id: Some(id),
                           task: String::from(description),
                           status: status == 1,
                       }
                   },
                   ).expect("Could not execute query");

        tasks
    }
}

pub fn run_prompter(my_todo: & mut Todo, conn: & mut PooledConn)  {

    loop {
        let prompt = "> ";
        std::io::stdout().write_all(prompt.as_bytes()).expect("Could not write out to stdout");
        std::io::stdout().flush().expect("Could not flush buffer");

        let mut input = String::new();
        std::io::stdin().read_line(& mut input).expect("Could not read line");
        
        let re = Regex::new(r#""([^"]*)"|\S+"#).expect("Could not create regex");

        let mut args: Vec<&str> = Vec::new();

        for cap in re.captures_iter(&input[..]) {
            if let Some(quoted) = cap.get(1) {
                args.push(quoted.as_str());
            }
            else {
                args.push(cap.get(0).unwrap().as_str());
            }
        }

        run(my_todo, args, conn);
    }
}


fn run(my_todo: & mut Todo, args: Vec<&str>, conn: & mut PooledConn) {

    if(args.len() == 0) {
        return;
    }

    let command = args[0];
    let command_result = match command {
        "add" => add_command_parser(my_todo, args, conn),
        "list" => print_tasks(my_todo, conn),
        "delete" => delete_command_parser(my_todo, args, conn),
        "update" => update_command_parser(my_todo, args, conn),
        "done" => done_command_parser(my_todo, args, conn),
        "exit" => {
            process::exit(0);
        }

        "help" | _ => print_help(),
    };

    if let Err(error) = command_result {
        println!("Error: {error}");
        println!("Use help command to see usage");
    } else {
        return;
    }
}

fn add_command_parser(my_todo: & mut Todo, args: Vec<&str>, conn: & mut PooledConn) -> Result<(),&'static str> {
    if let Some(task) = args.get(1) {

        let task = String::from(*task);
        my_todo.add_task(Task::new(task), conn);

        Ok(())
    } else {
        Err("Not enough arguments.")
    }
}

fn update_command_parser(my_todo: & mut Todo, args: Vec<&str>, conn: & mut PooledConn) -> Result<(), &'static str> {
    if let Some(task_id) = args.get(1) {
        if let Some(task) = args.get(2) {

            let index: Result<u64, _> = (*task_id).parse();

            match index {
                Ok(index) => {
                    return my_todo.update_task(index, *task, conn);
                }
                Err(_) => {
                    return Err("Could not parse index");
                }
            }

        } else {
            return Err("Not enough arguments");
        }
    } else {
        return Err("Not enough arguments");
    }
}

fn delete_command_parser(my_todo: & mut Todo, args: Vec<&str>, conn: & mut PooledConn) -> Result<(),& 'static str> {
    if let Some(task_id) = args.get(1) {
        let index: Result<u64, _> = (*task_id).parse();
        
        match index {
            Ok(index) => {
                return my_todo.delete_task(index, conn);
            }
            Err(_) => {
                return Err("Could not parse the argument as integer");
            }
        }
    } else {
        Err("Not enough arguments.")
    }
}


fn done_command_parser(my_todo: & mut Todo, args: Vec<&str>, conn: & mut PooledConn) -> Result<(),& 'static str> {
    if let Some(task_id) = args.get(1) {
        let index: Result<u64, _> = (*task_id).parse();
        
        match index {
            Ok(index) => {
                return my_todo.update_task_status(index, conn);
            }
            Err(_) => {
                return Err("Could not parse the argument as integer");
            }
        }
    } else {
        Err("Not enough arguments.")
    }
}

fn print_tasks (my_todo: & mut Todo, conn: & mut PooledConn) -> Result<(), & 'static str> {


    let mut task_list: Vec<Task> = my_todo.get_tasks(conn);
    if (task_list.len() == 0) {
        println!("Your task list is empty !! use \"add\" to add tasks to your list.");
        return Ok(())
    }

    println!("Your task list :");
    for current_task in task_list {

        if let Some(id) = current_task.id {
            let details = format!("{}. {}", id, current_task.task);

            if (current_task.status) {
                println!("X {details}");
            } else {
                println!("=> {details}");
            }
        }
        else {
            panic!("There was some error retreiving tasks...");
        }
    }

    Ok(())
}

fn print_help() -> Result<(), & 'static str> {
    let help: &str = "
Welcome to the todo_list application. 
structure of query: 
    command [arguments] 

supported commands: 
    add - Add a new task to the todo list, followed by a new task string. The task string should NOT be space separated. 

        usage: >add task_string

    list - Display the todo list 
        
        usage: >list

    delete - delete a task from the todo list, based on the task id provided by the user in the prompt. 

        usage: >delete task_id

    update - change the name of a task, followed by an integer number task id. 

        usage: >update task_id new_task_string 

    done - change the done status of a task from false to true, followed by an integer number task id. 
        
        usage: >done task_id 

    exit- exit the program. 
        
        usage: >exit

    help - display this help message. 
        
        usage: >help 

arguments: 
    task_id: the unique id assigned to each task. 

    task_string: the string for the task provided by the user. ";

    println!("{}", help);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_add() {
        let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
        let mut conn = pool.get_conn().unwrap();

        let my_todo = Todo::new("my_todo", & mut conn);
    }

    #[test]
    fn task_add() {
        let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
        let mut conn = pool.get_conn().unwrap();

        let mut my_todo = Todo::new("my_todo2", & mut conn);
        add_command_parser(& mut my_todo, vec!["add","sleep"], &mut conn);
    }

    #[test]
    fn get_tasks() {
        let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
        let mut conn = pool.get_conn().unwrap();

        let mut my_todo = Todo::new("my_todo", & mut conn);
        add_command_parser(& mut my_todo, vec!["add","eat"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","sleep"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","repeat"], &mut conn);

        print_tasks(& mut my_todo, & mut conn);
    }

    #[test]
    fn delete_task() {
        let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
        let mut conn = pool.get_conn().unwrap();

        let mut my_todo = Todo::new("my_todo", & mut conn);

        add_command_parser(& mut my_todo, vec!["add","eat"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","sleep"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","repeat"], &mut conn);

        delete_command_parser(& mut my_todo, vec!["delete","29"], &mut conn).expect("fucked up");
    }

    #[test]
    fn update_task() {
        let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
        let mut conn = pool.get_conn().unwrap();

        let mut my_todo = Todo::new("my_todo", & mut conn);

        add_command_parser(& mut my_todo, vec!["add","eat"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","sleep"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","repeat"], &mut conn);

        update_command_parser(& mut my_todo, vec!["update","35", "hey man"], &mut conn).expect("fucked up");
    }

    #[test]
    fn update_task_status() {
        let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
        let mut conn = pool.get_conn().unwrap();

        let mut my_todo = Todo::new("my_todo", & mut conn);

        add_command_parser(& mut my_todo, vec!["add","eat"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","sleep"], &mut conn);
        add_command_parser(& mut my_todo, vec!["add","repeat"], &mut conn);

        done_command_parser(& mut my_todo, vec!["done","44"], &mut conn).expect("fucked up");
    }
}
