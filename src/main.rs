use log::{trace};

mod database;

fn main() {
    let conn = match database::SQLiteConnection::new()
        .with_path("./db")
        .connect()
    {
        Ok(conn) => {
            trace!("Successfully connected to SQLite database.");
            conn
        },
        Err(e) => {
            panic!("No database connection available: {}", e);
        }
    };
}
