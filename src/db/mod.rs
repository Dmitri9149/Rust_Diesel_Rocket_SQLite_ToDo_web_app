use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;
/* use crate::db::schema::task::title;  */


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
/*    let db = "./testdb.sqlite3";   */
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_task(connection: &SqliteConnection, title: &str, done: &str) {
    let task = models::NewTask {title, done};

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");

}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
    .load::<models::Task>(connection)
    .expect("Error loading tasks")
}

pub fn delete_by_title(connection: &SqliteConnection, pattern: &str) {
        let tasks = schema::task::table;
/*        let num_deleted = diesel::delete(tasks.filter(tasks.title.like(pattern)))  */
        let num_deleted = diesel::delete(tasks)
        .execute(connection)
        .expect("Error deleting posts");
}
