#![allow(unused)]
use chrono::{DateTime, Local, TimeZone};
mod args;
use args::Args;
use clap::Parser;
use dirs::home_dir;
use std::fs::{self, rename};
use trash::os_limited::purge_all;
use trash::{TrashItem, delete};

pub fn purge() {}
//
// pub fn tidy_trash_directory() {}
//
pub fn move_to_trash(path: &std::path::Path) -> std::result::Result<(), trash::Error> {
    delete(path)
}

pub fn list_trash() {
    match trash::os_limited::list() {
        Ok(trash) => {
            for entry in trash {
                let time_deleted = Local
                    .timestamp_opt(entry.time_deleted, 0)
                    .single()
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown time".to_string());

                println!(
                    "Name: {}\nOriginal Location: {}\nDeleted At: {}\n",
                    entry.name.to_string_lossy(),
                    entry.original_parent.to_string_lossy(),
                    time_deleted
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to list trash entries: {e}");
        }
    }
}

fn main() {
    // parsing the args
    let args = Args::parse();

    let paths = args.get_files();
    let recursive = args.recursive;
    let force = args.force;
    let dir = args.dir;
    let ignore = args.ignore;

    if args.is_purge() {
        if let Err(e) = purge_all() {
            eprintln!("Error purging trash: {e}");
        }
        return;
    }

    // listing the trash directory if the list command is used
    if args.is_list() {
        list_trash();
        return;
    }

    // tidying the trash directory if the tidy command is used
    if args.is_tidy() {
        // tidy_trash_directory();
        return;
    }

    // Handle remove command (or default behavior when no subcommand)
    if args.is_remove() {
        // iterating over the paths
        for path in paths {
            if path.is_dir() && dir {
                if let Err(e) = fs::remove_dir(&path) {
                    eprintln!("Error removing directory: {e}")
                }
                continue;
            }

            if path.is_dir() && !recursive {
                if !force {
                    eprintln!("rmxd: cannot remove {path:?}: Is a directory");
                }
                continue;
            }

            if ignore {
                // not need of seperate function for this
                if let Err(e) = fs::remove_dir_all(&path) {
                    eprintln!("Error deleting with out moving to trash: {e}")
                }
            } else if let Err(e) = move_to_trash(&path) {
                eprintln!("Error moving to trash: {e}");
            }
        }
    }
}
