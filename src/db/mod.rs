use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| pamic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    models::NewTask {title};

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");

}