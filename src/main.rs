#![allow(unused)]
use chrono::{Local, TimeZone};
mod args;
use args::Args;
use args::Commands::RecoverAll;
use args::Commands::Tidy;
use clap::Parser;
use clap::builder::OsStr;
use std::error::Error;
use std::{fs, result};
use trash::os_limited::{self, purge_all, restore_all};
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

pub fn purge(name: &str) -> Result<(), trash::Error> {
    let content_to_remove: Vec<TrashItem> = trash::os_limited::list()
        .unwrap()
        .into_iter()
        .filter(|content| content.name == name)
        .collect();
    purge_all(content_to_remove)
}

pub fn list_specific_trash(seconds: i64) {
    let entries = trash::os_limited::list().unwrap();
    let now = Local::now().timestamp();
    for entry in entries {
        if now - entry.time_deleted < seconds {
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

pub fn tidy_trash(days: i64) -> Result<(), Box<dyn Error>> {
    let seconds: i64 = days * 86400;
    let entries = trash::os_limited::list()?;
    let now = Local::now().timestamp();
    for entry in entries {
        if now - entry.time_deleted > seconds {
            purge(&entry.name.to_string_lossy())?;
            println!("Purged: {}", entry.name.to_string_lossy());
        }
    }
    Ok(())
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
        let files_to_purge = args.get_purge_name();
        println!("got the files to purte{:#?}", files_to_purge);
        // if let Some(filename) = args.get_purge_name() {
        //     if let Err(e) = purge(filename) {
        //         eprintln!("Error removing the content from the bin: {e}")
        //     }
        // }
    }

    if args.is_recover_all() {
        let seconds = args.get_time_recover() * 86400;
        let mut content_to_recover = vec![];
        if seconds == 0 {
            restore_all(os_limited::list().unwrap());
        } else {
            let entries = trash::os_limited::list().unwrap();
            let now = Local::now().timestamp();
            for entry in entries {
                if now - entry.time_deleted < seconds {
                    content_to_recover.push(entry);
                }
            }
            trash::os_limited::restore_all(content_to_recover);
        }
    }

    // listing the trash directory if the list command is used
    if args.is_list() {
        let seconds = args.get_time_list() * 86400;
        if seconds == 0 {
            list_trash();
            return;
        } else {
            list_specific_trash(seconds);
        }
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
        let days = args.get_time_tidy();
        println!(
            "Warning: This will tidy the trash. \nAll the contents for the trash more then {days} days will me deleted permanently.\n  Do you want to proceed? (yes/no)"
        );
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Operation cancelled.");
            return;
        }

        if let Err(e) = tidy_trash(days) {
            eprintln!("Error tidying the trash: {e}");
        }
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
            } else if let Err(e) = delete(&path) {
                eprintln!("Error moving to trash: {e}");
            }
        }
    }
}
