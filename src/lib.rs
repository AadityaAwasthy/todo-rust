use std::{io::Write, error::Error};
use std::{process, sync::atomic::{AtomicU64, self}};
use regex::Regex;

pub struct Task {
    task: String,
    id: u64,
    status: bool,
}

static UNIQUE_ID: AtomicU64  = AtomicU64::new(1);

impl Task {
    fn new(task: String) -> Task {
        let id: u64 = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);
        Task{task, id, status:false}
    }
}

pub struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {tasks: Vec::new()}
    }

    fn add_task(& mut self, task: Task) {
        self.tasks.push(task);
    }

    fn delete_task(& mut self, id: u64) -> Result<(), &'static str> {
        if !(self.tasks.iter().any(|task| task.id == id)) {
            return Err("Invalid id");
        }

        self.tasks.retain(|task| task.id != id);
        Ok(())
    }

    fn update_task(& mut self, id: u64, task: &str ) -> Result<(), &'static str> {
        if !(self.tasks.iter().any(|task| task.id == id)) {
            return Err("Invalid id");
        }

        self.tasks.iter_mut().for_each(|current_task| {
            if current_task.id == id {
                current_task.task = String::from(task);
            }
        });

        return Ok(());
    }

    fn update_task_status(& mut self, id: u64) -> Result<(), &'static str> {
        if !(self.tasks.iter().any(|task| task.id == id)) {
            return Err("Invalid id");
        }

        self.tasks.iter_mut().for_each(|current_task| {
            if current_task.id == id {
                current_task.status = true;
            }
        });

        return Ok(());
    }

    fn get_tasks(& self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }
}

pub fn run_prompter(my_todo: & mut Todo)  {

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

        run(my_todo, args);
    }
}


fn run(my_todo: & mut Todo, args: Vec<&str>) {

    if(args.len() == 0) {
        return;
    }

    let command = args[0];
    let command_result = match command {
        "add" => add_command_parser(my_todo, args),
        "list" => print_tasks(my_todo),
        "delete" => delete_command_parser(my_todo, args),
        "update" => update_command_parser(my_todo, args),
        "done" => done_command_parser(my_todo, args),
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
//
fn add_command_parser(my_todo: & mut Todo, args: Vec<&str>) -> Result<(),&'static str> {
    if let Some(task) = args.get(1) {

        let task = String::from(*task);
        my_todo.add_task(Task::new(task));

        Ok(())
    } else {
        Err("Not enough arguments.")
    }
}

fn update_command_parser(my_todo: & mut Todo, args: Vec<&str>) -> Result<(), &'static str> {
    if let Some(task_id) = args.get(1) {
        if let Some(task) = args.get(2) {

            let index: Result<u64, _> = (*task_id).parse();

            match index {
                Ok(index) => {
                    return my_todo.update_task(index, *task);
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

fn delete_command_parser(my_todo: & mut Todo, args: Vec<&str>) -> Result<(),& 'static str> {
    if let Some(task_id) = args.get(1) {
        let index: Result<u64, _> = (*task_id).parse();
        
        match index {
            Ok(index) => {
                return my_todo.delete_task(index);
            }
            Err(_) => {
                return Err("Could not parse the argument as integer");
            }
        }
    } else {
        Err("Not enough arguments.")
    }
}


fn done_command_parser(my_todo: & mut Todo, args: Vec<&str>) -> Result<(),& 'static str> {
    if let Some(task_id) = args.get(1) {
        let index: Result<u64, _> = (*task_id).parse();
        
        match index {
            Ok(index) => {
                return my_todo.update_task_status(index);
            }
            Err(_) => {
                return Err("Could not parse the argument as integer");
            }
        }
    } else {
        Err("Not enough arguments.")
    }
}

fn print_tasks (my_todo: & mut Todo) -> Result<(), & 'static str> {


    let task_list: Vec<&Task> = my_todo.get_tasks().collect();
    if (task_list.len() == 0) {
        println!("Your task list is empty !! use \"add\" to add tasks to your list.");
        return Ok(())
    }

    println!("Your task list :");
    for current_task in task_list {
        let details = format!("{}. {}", current_task.id, current_task.task);

        if (current_task.status) {
            println!("X {details}");
        } else {
            println!("=> {details}");
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
