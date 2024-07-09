pub struct Todo {
    tasks: Vec<String>,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {tasks: Vec::new()}
    }

    pub fn add_task(& mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn print_tasks(& mut self) {
        println!("You have tasks :");

        for (serial_number, task) in self.tasks.iter().enumerate().map(|(a,b)| {(a + 1, b)}) {
            println!("{serial_number}. {task}.");
        }
    }
}
