use rusqlite::{params, types::Null, Connection, Statement, NO_PARAMS, Result};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use log::{debug, error, info, trace, warn};

// TODO: Find out why this is required?!
#[path = "sql.rs"]
mod sql;

//
// TODO: should probably place these files in the users home directory for each platform.
//
/// Default path for the sqlite database.
const DEFAULT_PATH: &'static str = "./data";

/// Default filename for the sqlite database.
const DEFAULT_DATABASE_NAME: &'static str = "mtg-deck-builder-db.sqlite3";

pub struct SQLiteConnection {
    path: PathBuf,
    filename: PathBuf,
}

impl SQLiteConnection {
    pub fn new() -> SQLiteConnection {
        SQLiteConnection {
            path: PathBuf::from(DEFAULT_PATH),
            filename: PathBuf::from(DEFAULT_DATABASE_NAME),
        }
    }

    /// Optionally set a custom database path.
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = PathBuf::from(path);
        self
    }

    /// Optionally set a custom database filename.
    pub fn with_filname(mut self, filename: &str) -> Self {
        self.filename = PathBuf::from(filename);
        self
    }

    pub fn connect(self) -> Result<Connection> {
        if !self.path.exists() {
            match std::fs::create_dir_all(&self.path) {
                Ok(_) => {
                    trace!(
                        "Created directory structure: {}",
                        self.path.to_str().unwrap()
                    );
                }
                Err(e) => {
                    error!("Failed to connect to database: {}", e);
                }
            }
        }
        Connection::open(&self.path.join(&self.filename))
    }
}

/// Struct to store the database connection and any prepared statements which are loaded on demand.
pub struct Manager<'a> {
    conn: &'a Connection,

    // Prepared statements
    insert_card_stmt: Option<Statement<'a>>,
}

impl<'a> Manager<'a> {
    /// Create a new database manager object with with the supplied SQLite connection.
    pub fn new(conn: &'a Connection) -> Manager<'a> {
        Manager {
            conn: conn,

            // Prepared statements
            insert_card_stmt: None,
        }
    }

    /// Creates database tables for local storage of card data
    pub fn create_tables(&self) {
        match self
            .conn
            .execute_batch(sql::CREATE_DATABASE_TABLES_STMT)
        {
            Ok(_) => trace!("Successfully created database tables."),
            Err(e) => error!("Failed to create database tables: {}", e),
        };
    }

    /// Adds a card to your local collection
    pub fn insert_card(&mut self, params: &[&dyn rusqlite::ToSql]) -> Result<()> {
        if let None = &self.insert_card_stmt {
            self.insert_card_stmt = Some(self.conn.prepare(sql::INSERT_CARD_STMT)?)
        }
        match self.insert_card_stmt.as_mut().unwrap().execute(
            params,
        ) {
            Ok(_) => trace!("Successfully added card data."),
            Err(e) => warn!("Failed to insert card data: {}", e),
        };
        Ok(())
    }
}
