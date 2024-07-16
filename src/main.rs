use todo::{Todo,run_prompter};
use mysql::*;
use mysql::prelude::*;

fn main() {

    let pool = Pool::new("mysql://root:3021@localhost:3306/todo").unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut my_todo = Todo{
        id:1,
        name: String::from("starter"),
    };

    run_prompter(& mut my_todo, & mut conn);
}

