use std::io::Write;
use todo::Todo;

fn main() {
    let mut my_todo = Todo::new();
    
    loop {
        let input = get_prompt("What do you want to do ? (add, print,  exit): ");
        match &input[..] {
            "add" => get_task(& mut my_todo),
            "print" => print_task(& my_todo),
            "exit" => return,
            _ => {
                println!("Please enter a command !");
                continue;
            },
        };
    }
}

fn get_prompt(line: &str) -> String {
    let mut input = String::new();

    std::io::stdout().write_all(line.as_bytes()).expect("Could not write out to stdout");
    std::io::stdout().flush().expect("Could not flush buffer");
    std::io::stdin().read_line(& mut input).expect("Could not read line");
    input.pop();

    input
}

fn get_task(my_todo: & mut Todo)  {
    my_todo.add_task(get_prompt("Enter a task: "));
}

fn print_task (my_todo: & Todo){
    let tasks_iterator = my_todo.print_tasks();

    println!("You have tasks : ");
    for (serial_num, task) in tasks_iterator.enumerate().map(|(a,b)| {(a + 1, b)}) {
        println!("{serial_num}. {task}");
    }

}
