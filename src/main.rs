use std::io::Write;
use todo::Todo;

fn main() {
    let mut my_todo = Todo::new();
    
    loop {
        println!("What do you want to do ? (add, print,  exit): ");
        let mut input = String::new();

        std::io::stdin().read_line(& mut input).expect("Could not read line from stdin");
        input.pop();

        match &input[..] {
            "add" => add_task(& mut my_todo),
            "print" => my_todo.print_tasks(),
            "exit" => return,
            _ => {
                println!("Please enter a command !");
                continue;
            },
        };
    }
}

fn add_task(my_todo: & mut Todo)  {
    let mut input = String::new();

    std::io::stdout().write_all("Enter a task to add: ".as_bytes()).expect("Could not write out to stdout");
    std::io::stdout().flush().expect("Could not flush buffer");
    std::io::stdin().read_line(& mut input).expect("Could not read line");
    input.pop();

    my_todo.add_task(input);
}
