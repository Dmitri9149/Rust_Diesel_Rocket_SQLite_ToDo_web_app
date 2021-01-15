use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask {title};

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");

}

fn show_tasks(args: &[String]) {
    if args.len()  > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }
    let conn = establish_conection();
    println!("TASKS\n--------");
    for task in query_task(&conn) {
        println!("{}", task.title);
    }
}





