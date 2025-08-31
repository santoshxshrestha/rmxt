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

pub fn list_specific_trash(seconds: i64) -> Result<(), trash::Error> {
    let entries = trash::os_limited::list()?;
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
    Ok(())
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

pub fn tidy_trash(days: i64) {
    let seconds: i64 = days * 86400;
    let now = Local::now().timestamp();
    let content_to_purge = trash::os_limited::list()
        .unwrap()
        .into_iter()
        .filter(|item| item.time_deleted < seconds)
        .collect::<Vec<TrashItem>>();

    if !content_to_purge.is_empty() {
        if let Err(e) = trash::os_limited::purge_all(content_to_purge) {
            eprintln!("Error purging items: {e}");
        } else {
            println!("No items found to purge older than {days} days");
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
        let names = args.get_purge_name();
        let content_to_purge = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|item| names.contains(&item.name.to_string_lossy().to_string()))
            .collect::<Vec<TrashItem>>();

        if !content_to_purge.is_empty() {
            if let Err(e) = trash::os_limited::purge_all(content_to_purge) {
                eprintln!("Error purging items: {e}");
            }
        } else {
            println!("No items found to purge with such names");
        }
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
            if let Err(e) = trash::os_limited::restore_all(content_to_recover) {
                eprintln!("Error recovering items: {e}");
            }
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
            return;
        }
        return;
    }

    // recovering files from trash if the recover command is used
    if args.is_recover() {
        let names = args.get_recover_name();
        let content_to_recover = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|item| names.contains(&item.name.to_string_lossy().to_string()))
            .collect::<Vec<TrashItem>>();
        if !content_to_recover.is_empty() {
            if let Err(e) = trash::os_limited::restore_all(content_to_recover) {
                eprintln!("Error recovering items: {e}");
            }
        } else {
            println!("No items found to recover with such names");
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

        tidy_trash(days);
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
