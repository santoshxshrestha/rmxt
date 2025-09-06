use chrono::{Local, TimeZone};
use colored::Colorize;
use tabled::{
    Table, Tabled,
    settings::{Alignment, Style, object::Columns},
};
use trash::os_limited::restore_all;
mod args;
use args::Args;
use clap::Parser;
use std::fs;
use trash::os_limited;
use trash::{TrashItem, delete};

#[derive(Tabled)]
pub struct List {
    name: String,
    original_location: String,
    deleted_at: String,
}

impl List {
    pub fn new(name: String, original_location: String, deleted_at: String) -> Self {
        Self {
            name,
            original_location,
            deleted_at,
        }
    }
}

pub fn list_specific_trash(seconds: i64) -> Result<(), trash::Error> {
    let mut list: Vec<List> = vec![];
    let entries = os_limited::list()?;
    let now = Local::now().timestamp();
    let cutoff_time = now - seconds;
    for entry in entries {
        if entry.time_deleted >= cutoff_time {
            let time_deleted = Local
                .timestamp_opt(entry.time_deleted, 0)
                .single()
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "Unknown time".to_string());
            list.push(List::new(
                entry.name.to_string_lossy().to_string(),
                entry.original_path().to_string_lossy().to_string(),
                time_deleted,
            ));
        }
    }
    let mut table = Table::new(&list);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());
    println!("{table}");
    Ok(())
}

pub fn list_trash() {
    let mut list: Vec<List> = vec![];
    match os_limited::list() {
        Ok(trash) => {
            for entry in trash {
                let time_deleted = Local
                    .timestamp_opt(entry.time_deleted, 0)
                    .single()
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown time".to_string());

                list.push(List::new(
                    entry.name.to_string_lossy().to_string(),
                    entry.original_path().to_string_lossy().to_string(),
                    time_deleted,
                ))
            }
            let mut table = Table::new(&list);
            table.with(Style::modern());
            table.modify(Columns::first(), Alignment::right());
            println!("{table}");
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to list trash entries: {e}").red())
        }
    }
}

pub fn tidy_trash(days: i64) -> Result<(), trash::Error> {
    let seconds: i64 = days * 86400;
    let cutoff_time = Local::now().timestamp() - seconds;
    let content_to_purge = trash::os_limited::list()?
        .into_iter()
        .filter(|item| item.time_deleted < cutoff_time)
        .collect::<Vec<TrashItem>>();

    if !content_to_purge.is_empty() {
        let purge_count = content_to_purge.len();
        if let Err(e) = os_limited::purge_all(content_to_purge) {
            eprintln!("{}", format!("Error purging items: {e}").red());
        } else {
            println!("Purged {purge_count} items older than {days} days");
        }
    } else {
        println!("No items found to purge older than {days} days");
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
        let names = args.get_purge_name();
        let content_to_purge = match trash::os_limited::list() {
            Ok(item) => item
                .into_iter()
                .filter(|item| names.contains(&item.name.to_string_lossy().to_string()))
                .collect::<Vec<TrashItem>>(),
            Err(e) => {
                eprintln!("{}", format!("Error listing items: {e} ").red());
                return;
            }
        };
        if !content_to_purge.is_empty() {
            if let Err(e) = trash::os_limited::purge_all(content_to_purge) {
                eprintln!("{}", format!("Error purging items: {e}").red());
            }
        } else {
            println!("No items found to purge with such names");
        }
    }

    if args.is_recover_all() {
        let seconds = args.get_time_recover() * 86400;
        let mut content_to_recover = vec![];
        if seconds == 0 {
            match os_limited::list() {
                Ok(items) => {
                    if let Err(e) = restore_all(items) {
                        eprintln!("{}", format!("Error recovering items: {e}").red());
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("Error listing items: {e}").red());
                }
            }
        } else {
            let Ok(entries) = os_limited::list() else {
                eprintln!("{}", "Error listing items ".to_string().red());
                return;
            };
            let now = Local::now().timestamp();
            let cutoff_time = now - seconds;
            for entry in entries {
                if entry.time_deleted >= cutoff_time {
                    content_to_recover.push(entry);
                }
            }
            if let Err(e) = trash::os_limited::restore_all(content_to_recover) {
                eprintln!("{}", format!("Error recovering items: {e}").red());
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
            if let Err(e) = list_specific_trash(seconds) {
                eprintln!("{}", format!("Failed to list trash entries: {e}").red());
            }
            return;
        }
    }

    // recovering files from trash if the recover command is used
    if args.is_recover() {
        let names = args.get_recover_name();
        let content_to_recover = match os_limited::list() {
            Ok(item) => item
                .into_iter()
                .filter(|item| names.contains(&item.name.to_string_lossy().to_string()))
                .collect::<Vec<TrashItem>>(),
            Err(e) => {
                eprintln!("Error listing items: {e} ");

                return;
            }
        };

        if !content_to_recover.is_empty() {
            if let Err(e) = trash::os_limited::restore_all(content_to_recover) {
                eprintln!("{}", format!("Error recovering items: {e}").red());
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
            "           {}This will tidy the trash.
All the contents from the trash more then {days} days will be deleted permanently 
            Do you want to proceed? (yes/no)
",
            "Warning: ".to_string().red()
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
            eprintln!("{}", format!("Error tidying trash: {e}").red());
        }
        return;
    }

    // Handle remove command (or default behavior when no subcommand)
    if args.is_remove() {
        // iterating over the paths
        for path in paths {
            if !path.exists() {
                eprintln!(
                    "{}",
                    format!("rmxd: cannot remove {path:?}: No such file or directory").red()
                )
            }
            if path.is_dir() && dir {
                if let Err(e) = fs::remove_dir(&path) {
                    eprintln!("{}", format!("Error removing directory: {e}").red())
                }
                continue;
            }

            if path.is_dir() && !recursive {
                if !force {
                    eprintln!(
                        "{}",
                        format!("rmxd: cannot remove {path:?}: Is a directory").red()
                    );
                }
                continue;
            }

            match (ignore, path.is_dir(), path.is_file()) {
                (true, true, _) => {
                    if let Err(e) = fs::remove_dir_all(&path) {
                        eprintln!(
                            "{}",
                            format!("Error deleting with out moving to trash: {e}").red()
                        )
                    }
                }
                (true, false, true) => {
                    if let Err(e) = fs::remove_file(&path) {
                        eprintln!(
                            "{}",
                            format!("Error deleting with out moving to trash: {e}").red()
                        )
                    }
                }
                (false, _, _) => {
                    if let Err(e) = delete(&path) {
                        eprintln!("{}", format!("Error moving to trash: {e}").red());
                    }
                }
                _ => {}
            }
        }
    }
}
