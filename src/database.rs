use rusqlite::{Connection, Statement, Result};
use std::path::{Path, PathBuf};

use log::trace;

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
