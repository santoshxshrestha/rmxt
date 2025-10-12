use chrono::{Local, TimeZone};
use colored::Colorize;
use std::path::Path;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Style};
use tabled::{Table, Tabled};
use trash::os_limited::restore_all;
mod args;
use args::Args;
use clap::Parser;
use std::{fs, path::PathBuf};
use trash::os_limited;
use trash::{TrashItem, delete};

#[derive(Tabled)]
struct List {
    name: String,
    original_location: String,
    deleted_at: String,
}

impl List {
    fn new(name: String, original_location: String, deleted_at: String) -> Self {
        Self {
            name,
            original_location,
            deleted_at,
        }
    }
}

fn list_specific_trash(seconds: i64) -> Result<Vec<List>, trash::Error> {
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
    return Ok(list);
}

fn list_trash() -> Result<Vec<List>, trash::Error> {
    let mut list: Vec<List> = vec![];
    let trash = os_limited::list()?;
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
    return Ok(list);
}

fn display_table(list: Vec<List>) {
    let mut table = Table::new(&list);
    table.with(Style::rounded());
    table.modify(Columns::first(), Alignment::right());
    println!("{table}");
}

fn tidy_trash(days: i64) -> Result<(), trash::Error> {
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
fn resolve_conflict(path: &PathBuf) -> std::io::Result<()> {
    let name = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => {
            eprintln!("{}", "Path does not have a valid filename".red());
            return Ok(());
        }
    };

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");

    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| name.clone());

    let extension = path
        .extension()
        .map(|ext| format!(".{}", ext.to_string_lossy()))
        .unwrap_or_default();

    let new_name = format!("{stem}_{timestamp}{extension}");

    eprintln!(
        "{}",
        format!(
            "Conflict detected: '{}' already exists in trash. Would be renamed to: '{}'",
            name, new_name
        )
        .yellow()
    );

    fs::rename(path, &new_name)?;

    if let Err(e) = delete(&new_name) {
        eprintln!(
            "{}",
            format!("Error moving {} to trash: {e}", path.display()).red()
        );
    }

    Ok(())
}

fn check_conflict(path: &Path) -> bool {
    let name = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => {
            eprintln!("{}", "Path does not have a valid filename".red());
            return false;
        }
    };

    let trash_list = match os_limited::list() {
        Ok(items) => items,
        Err(e) => {
            eprintln!("{}", format!("Error listing trash items: {e}").red());
            return false;
        }
    };

    trash_list
        .iter()
        .any(|item| item.name.to_string_lossy() == name)
}

fn main() {
    let args = Args::parse();

    let paths = args.get_items();
    let recursive = args.recursive;
    let force = args.force;
    let dir = args.dir;
    let permanent = args.permanent;

    // Handling the case where no paths are provided and no subcommand is used
    if paths.is_empty() && args.command.is_none() {
        eprintln!("{}", "rmxt: missing operand".red());
        eprintln!("{}", "Try 'rmxt --help' for more information.".yellow());
        return;
    }

    // purging files from trash if the purge command is used
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

    // recovering all files from trash if the recover-all command is used
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
            match list_trash() {
                Ok(list) => display_table(list),
                Err(e) => eprintln!("{}", format!("Failed to list trash entries: {e}").red()),
            }

            return;
        } else {
            match list_specific_trash(seconds) {
                Ok(list) => display_table(list),
                Err(e) => eprintln!("{}", format!("Failed to list trash entries: {e}").red()),
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
                    format!("rmxt: cannot remove {path:?}: No such file or directory").red()
                );
                continue;
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
                        format!("rmxt: cannot remove {path:?}: Is a directory").red()
                    );
                }
                continue;
            }

            match (permanent, dir, path.is_dir(), path.is_file(), recursive) {
                (true, false, true, _, true) => {
                    if let Err(e) = fs::remove_dir_all(&path) {
                        eprintln!(
                            "{}",
                            format!("Error deleting without moving to trash: {e}").red()
                        )
                    }
                }
                (true, true, true, _, false) => {
                    if let Err(e) = fs::remove_dir(&path) {
                        eprintln!(
                            "{}",
                            format!("Error deleting without moving to trash: {e}").red()
                        )
                    }
                }
                (true, _, false, true, _) => {
                    if let Err(e) = fs::remove_file(&path) {
                        eprintln!(
                            "{}",
                            format!("Error deleting without moving to trash: {e}").red()
                        )
                    }
                }
                (false, _, _, _, _) => {
                    match check_conflict(&path) {
                        true => {
                            if let Err(e) = resolve_conflict(&path) {
                                eprintln!("{}", format!("Error resolving conflict: {e}").red());
                            }
                        }
                        false => {
                            if let Err(e) = delete(&path) {
                                eprintln!(
                                    "{}",
                                    format!("Error moving {} to trash: {e}", path.display()).red()
                                );
                            }
                        }
                    };
                }
                _ => {}
            }
        }
    }
}
