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

    pub fn print_tasks(& self) -> impl Iterator<Item = &String> {
        self.tasks.iter()
    }
}
