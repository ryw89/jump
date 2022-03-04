use std::error::Error;

use crate::db::{get_conn, Dir};
use crate::score::score;

/// Find the best-matching directory and print.
pub fn jump(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let conn = get_conn()?;

    let sql: &str = "SELECT id, dir, access_count, last_accessed
                     FROM dir";

    let mut statement = conn.prepare(sql)?;

    let dir_iter = statement.query_map([], |row| {
        Ok(Dir {
            id: row.get(0)?,
            dir: row.get(1)?,
            access_count: row.get(2)?,
            last_accessed: row.get(3)?,
        })
    })?;

    // Iterate through all 'dir' rows and score them
    let mut max_score_dir = None;
    let mut max_score = 0;
    for d in dir_iter {
        let dir = d.as_ref().unwrap().clone().dir;
        let s = score(&args, &d?);
        if s > max_score {
            max_score = s;
            max_score_dir = Some(dir);
        }
    }

    // Print out max-scored dir -- This will be ingested by the shell.
    // (See the j() function in shell.rs.)
    match max_score_dir {
        Some(d) => println!("{}", d),
        None => return Err(String::from("db is empty {}").into()),
    }

    Ok(())
}
