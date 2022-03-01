use std::error::Error;
use std::fs;

use rusqlite::{named_params, Connection, Transaction};

use crate::dirs::get_data_dir;

/// Representation of a row in the 'dir' table.
#[derive(Clone)]
pub struct Dir {
    pub id: i64,
    pub dir: String,
    pub access_count: i64,
    pub last_accessed: i64,
}

#[derive(Clone)]
pub struct DirId {
    id: Option<i64>,
}

/// Get sqlite connection
pub fn get_conn() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(get_data_dir().join("jump.db"));

    match conn {
        Ok(c) => Ok(c),
        Err(e) => Err(Box::new(e)),
    }
}

/// Initialize sqlite database with needed tables.
pub fn init_db() -> Result<(), Box<dyn Error>> {
    let data_dir = get_data_dir();

    // Create data directory if needed
    fs::create_dir_all(data_dir)?;

    let conn = get_conn()?;

    // Main table of directories traversed by shell. (Note that
    // 'last_accessed' should be in Unix time.)
    let dir = "CREATE TABLE IF NOT EXISTS dir (
                   id INTEGER PRIMARY KEY NOT NULL,
                   dir TEXT NOT NULL,
                   access_count INTEGER NOT NULL,
                   last_accessed INTEGER NOT NULL
               );";

    // Table of most recent searches.
    let last_searches = "CREATE TABLE IF NOT EXISTS last_searches (
                   dir_id INTEGER NOT NULL,
                   query TEXT NOT NULl,
                   FOREIGN KEY(dir_id) REFERENCES dir(id)
               );";

    for s in &[dir, last_searches] {
        match conn.execute(s, []) {
            Ok(_) => {}
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}

/// Update or insert a 'dir' row.
pub fn update_or_insert_dir(dir: &Dir) -> Result<(), Box<dyn Error>> {
    let mut conn = get_conn()?;
    let tx = conn.transaction()?;
    exec_insert_dir_sql(dir, &tx)?;
    tx.commit()?;

    Ok(())
}

/// Execute SQL statement for inserting a 'dir' row.
fn exec_insert_dir_sql(dir: &Dir, tx: &Transaction) -> Result<(), Box<dyn Error>> {
    let sql: &str = "INSERT OR REPLACE INTO dir (id, dir, access_count, last_accessed)
                     VALUES (:id, :dir, :access_count, :last_accessed)";

    let mut statement = tx.prepare(sql)?;

    statement.execute(named_params! {
        ":id": dir.id,
        ":dir": dir.dir,
        ":access_count": dir.access_count,
        ":last_accessed": dir.last_accessed,
    })?;

    Ok(())
}

/// Fetch a 'dir' row from the database using its name.
pub fn get_dir_by_name(dir: &str) -> Result<Dir, Box<dyn Error>> {
    let conn = get_conn()?;

    let sql: &str = "SELECT id, dir, access_count, last_accessed
                     FROM dir
                     WHERE dir=:dir
                     LIMIT 1";

    let mut statement = conn.prepare(sql)?;

    let dir_iter = statement.query_map(named_params! {":dir": dir}, |row| {
        Ok(Dir {
            id: row.get(0)?,
            dir: row.get(1)?,
            access_count: row.get(2)?,
            last_accessed: row.get(3)?,
        })
    })?;

    for (i, d) in dir_iter.enumerate() {
        if i == 0 {
            return Ok(d?);
        }
    }

    Err(format!("No results found for directory {}", dir).into())
}

/// Generate a new Dir.
pub fn make_new_dir(dir: &str) -> Result<Dir, Box<dyn Error>> {
    let conn = get_conn()?;

    let sql: &str = "SELECT MAX(id)
                     FROM dir";

    let mut statement = conn.prepare(sql)?;

    let dir_iter = statement.query_map([], |row| Ok(DirId { id: row.get(0)? }))?;

    let mut id: i64 = -1;
    for (i, d) in dir_iter.enumerate() {
        // Increment the max index
        if i == 0 {
            id = match d?.id {
                // New ID is max ID plus one
                Some(i) => i + 1,
                // No IDs in db, so first id is 1
                None => 1,
            };
        }
    }

    // Sanity check -- This should not be reached
    if id < 0 {
        panic!("Generated ID for new row is less than zero.");
    }

    Ok(Dir {
        id,
        dir: dir.to_string(),
        access_count: 0,
        last_accessed: 0,
    })
}
