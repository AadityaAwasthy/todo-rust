use std::{io::Write, error::Error};
use std::{process, sync::atomic::{AtomicU64, self}};

pub struct Task {
    task: String,
    id: u64,
}

static UNIQUE_ID: AtomicU64  = AtomicU64::new(1);

impl Task {
    fn new(task: String) -> Task {
        let id: u64 = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);
        Task{task, id}
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

    // pub fn delete_task(& mut self, id: u64) {
    //     self.tasks.iter().filter(|task| task.id != id).collect();
    // }

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

        let arguments: Vec<&str> = input.split_whitespace().collect();
        run(my_todo, arguments);
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

fn print_tasks (my_todo: & mut Todo) -> Result<(), & 'static str> {


    let task_list: Vec<&Task> = my_todo.get_tasks().collect();
    if (task_list.len() == 0) {
        println!("Your task list is empty !! use \"add\" to add tasks to your list.");
        return Ok(())
    }

    println!("Your task list :");
    for current_task in task_list {
        println!("{}. {}", current_task.id, current_task.task);
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

    show - Display the todo list 
        
        usage: >show

    delete - delete a task from the todo list, based on the task id provided by the user in the prompt. 

        usage: >delete task_id

    update - change the name of a task, followed by an integer number task id. 

        usage: >update task_id new_task_string 

    done - change the done status of a task from false to true, follwed by an integer number task id. 
        
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
//
// pub fn delete_task(my_todo: & mut Todo) {
//     let mut input_index = get_prompt("Please enter the index of task to remove: ").parse::<usize>().unwrap();
//
//     input_index -= 1;
//
//     match my_todo.delete_task(input_index) {
//         Ok(removed_task) => println!("Removed task {}: {removed_task}",input_index + 1),
//         Err(error) => println!("Error: {error}"),
//     }
//
//     return;
// }
//
// pub fn print_task (my_todo: & Todo){
//     let tasks_iterator = my_todo.print_tasks();
//
//     println!("You have tasks : ");
//     for (serial_num, task) in tasks_iterator.enumerate().map(|(a,b)| {(a + 1, b)}) {
//         println!("{serial_num}. {task}");
//     }
//
// }
//
//
