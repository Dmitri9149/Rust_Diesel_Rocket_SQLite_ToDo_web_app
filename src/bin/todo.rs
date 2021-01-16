use std::env;
use mytodo::db::{create_task, query_task, establish_connection};

fn help() {
    println!("subcommands:");
    println!("   new<title>: create new task");
    println!(" show: get the list of all tasks");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        help();
        return;
    }

    println!("arg[0]-> {}; args[1] -> {}", &args[0], &args[1]);

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        _ => help(),
    }

}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0], &args[]);

}


fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn) {
        println!("title -> {};   done -> {}", task.title, task.done);
    }
}















