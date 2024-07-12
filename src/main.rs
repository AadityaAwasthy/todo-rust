use todo::{Todo,run_prompter};

fn main() {
    let mut my_todo = Todo::new();
    run_prompter(& mut my_todo);
}

