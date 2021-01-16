use std::env;
use mytodo::db::{create_task, query_task, establish_connection, delete_by_title};

fn help() {
    println!("subcommands:");
    println!("");
    println!("new<title>: create new task");
    println!("show: get the list of all tasks");
    println!("delete: delete 'tasks' by stating 'like name of task string'; be carefull!\n");
    println!("'learn C' and 'learn C++' will both deleted by pattern 'learn C'");
    println!("done: mark a task as 'done', it is need to know the id of the task\n");
    println!("that is why it is recommended to use 'show' first to gut the id");
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
        "delete" => delete_task(&args[2..]),
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
    create_task(&conn, &args[0], "pending");

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

fn delete_task(args: &[String]) {
    if args.len() < 1 {
        println!("delete: missing <title> like string");
        help();
        return;
    }

    let conn = establish_connection();
    let pattern = format!("%{}%", &args[0]);
    let num_deleted = delete_by_title(&conn, &pattern);

    println!("Deleted {} posts", num_deleted);

}

fn done(args: &[String])  {
    if args.len() < 1 {
        println!("done: missing 'id' of the task to update");
        help();
        return;
    }

    let conn = establish_connection();
    let id = &args[0].parse::<i32>().expect("Invalid ID");
    println!("This is {id}", id);
}











