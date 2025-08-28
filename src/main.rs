mod args;
use args::Args;
use clap::Parser;
use dirs::home_dir;
use std::fs::{self, rename};
use walkdir::WalkDir;

pub fn get_trash_directory() -> std::path::PathBuf {
    match home_dir() {
        Some(dir) => dir.join(".trash/"),
        None => panic!("Could not find home directory"),
    }
}

pub fn tidy_trash_directory() {
    let trash = get_trash_directory();

    match fs::metadata(&trash) {
        Ok(metadata) => {
            if metadata.is_dir() {
                match fs::remove_dir_all(&trash) {
                    Ok(_) => {
                        println!("Trash directory cleaned.");

                        if let Err(e) = fs::create_dir_all(&trash) {
                            eprintln!("Error recreating trash directory: {e}");
                        }
                    }
                    Err(e) => eprintln!("Error tidying trash directory: {e}"),
                }
            } else {
                eprintln!("Path exists but is not a directory.");
            }
        }
        Err(e) => {
            println!("Trash directory does not exist or cannot be accessed: {e}");
            println!("Creating trash directory at: {:?}", &trash);

            if let Err(e) = fs::create_dir_all(&trash) {
                eprintln!("Error creating trash directory: {e}");
            }
        }
    }
}

pub fn move_to_trash(path: &std::path::Path) -> Result<(), std::io::Error> {
    let trash = get_trash_directory();
    let file_name = match path.file_name() {
        Some(name) => name,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid file name",
            ));
        }
    };

    let mut new_path = trash.join(file_name);
    let mut count = 1;

    while new_path.exists() {
        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let extension = path
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default();
        new_path = trash.join(format!("{file_stem}-{count}{extension}"));
        count += 1;
    }
    rename(path, new_path)
}

pub fn permanently_delete(path: &std::path::Path) -> Result<(), std::io::Error> {
    if let Err(e) = fs::remove_dir_all(path) {
        Err(e)
    } else {
        Ok(())
    }
}

pub fn list_trash() {
    let trash = get_trash_directory();
    match WalkDir::new(&trash)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(entries) => {
            for entry in entries {
                match entry.path().to_str() {
                    Some(path) => println!("{path}"),
                    None => eprintln!("Error: Unable to convert path to string"),
                }
            }
        }
        Err(e) => eprintln!("Error: Failed to read trash directory - {e}"),
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

    let trash = get_trash_directory();

    // listing the trash directory if the list command is used
    if args.is_list() {
        list_trash();
        return;
    }

    // tidying the trash directory if the tidy command is used
    if args.is_tidy() {
        tidy_trash_directory();
        return;
    }

    // Handle remove command (or default behavior when no subcommand)
    if args.is_remove() {
        // creating the trash directory if it doesn't exist
        if !fs::exists(&trash).unwrap() {
            fs::create_dir(&trash).unwrap();
        }

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
                if let Err(e) = permanently_delete(&path) {
                    eprintln!("Error moving to trash: {e}");
                }
            } else if let Err(e) = move_to_trash(&path) {
                eprintln!("Error moving to trash: {e}");
            }
        }
    }
}
