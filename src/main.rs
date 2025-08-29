#![allow(unused)]
use chrono::{DateTime, Local, TimeZone};
mod args;
use args::Args;
use clap::Parser;
use dirs::home_dir;
use std::fs::{self, rename};
use trash::os_limited::{purge_all, restore_all};
use trash::{TrashItem, delete};

pub fn recover_from_trash(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let list = trash::os_limited::list()?;
    let items_to_restore: Vec<_> = list.into_iter().filter(|item| item.name == name).collect();

    if !items_to_restore.is_empty() {
        restore_all(items_to_restore)?;
        println!("Recovered '{name}'");
    } else {
        println!("No items found to recover with the name '{name}'");
    }

    Ok(())
}

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
        match trash::os_limited::list() {
            Ok(items) => {
                if let Err(e) = purge_all(items) {
                    eprintln!("Error purging trash: {e}");
                }
            }
            Err(e) => {
                eprintln!("Error listing trash for purge: {e}");
            }
        }
        return;
    }

    // listing the trash directory if the list command is used
    if args.is_list() {
        list_trash();
        return;
    }

    // recovering files from trash if the recover command is used
    if args.is_recover() {
        if let Some(name) = args.get_recover_name() {
            if let Err(e) = recover_from_trash(name) {
                eprintln!("Error recovering from trash: {e}");
            }
        }
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
