mod db;
mod dirs;
mod jump;
mod macros;
mod shell;

use std::env::current_dir;
use std::process;
use std::time::SystemTime;
use structopt::StructOpt;

use crate::db::*;
use crate::jump::jump;
use crate::shell::zsh;

/// Jump between directories.
#[derive(StructOpt, Debug)]
#[structopt(name = "jump")]
struct Opt {
    /// Directory to jump to
    dir: Vec<String>,

    /// Hook for shells when changing directories.
    #[structopt(short, long)]
    chdir: bool,

    /// Initialize database.
    #[structopt(short, long)]
    init: bool,

    /// Just print source code for shell. Supports bash and zsh.
    #[structopt(short, long)]
    shell: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    // Prioritize -s, --shell arg.
    if opt.shell.is_some() {
        let sh = opt.shell.as_ref().unwrap();
        if sh == "zsh" {
            zsh();
        } else {
            bad_exit!(format!("Unsupported shell: {}", sh));
        }
        process::exit(0);
    }

    // Next, prioritize initialization of db with -i, --init.
    if opt.init {
        match init_db() {
            Ok(_) => (),
            Err(e) => bad_exit!(format!("error: {}", e)),
        }

        println!("Initialize jump sqlite database.");
        process::exit(0);
    }

    // Shell 'change directory' hook
    if opt.chdir {
        let dir = current_dir().unwrap();

        // Test sqlite db connection.
        match get_conn() {
            Ok(_) => (),
            Err(_) => bad_exit!(
                "The jump database does not appear to be initialized. Use jump --init to do so."
            ),
        }

        // Fetch data for this directory, or initialize a new Dir
        // struct.
        let mut dir = match get_dir_by_name(dir.to_str().unwrap()) {
            Ok(d) => d,
            Err(_) => make_new_dir(dir.to_str().unwrap()).unwrap(),
        };

        // Increment access count & update access time
        let unix_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        dir.last_accessed = unix_time;
        dir.access_count += 1;

        // Insert or update dir
        match update_or_insert_dir(&dir) {
            Ok(_) => (),
            Err(e) => bad_exit!(format!("error: {}", e)),
        }

        process::exit(0);
    }

    // Main 'jump' logic -- Only reached if --chdir, --init, or
    // --shell weren't called
    if opt.dir.is_empty() {
        bad_exit!("jump: no arguments supplied")
    }

    match jump(opt.dir) {
        Ok(_) => (),
        Err(_) => {
            bad_exit!("No entries in jump database. Try changing directories first.".to_string())
        }
    }
}
