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
    let new_path = trash.join(file_name);
    rename(path, new_path)
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

    let paths = args.file;
    let recursive = args.recursive;
    let force = args.force;
    let tidy = args.tidy;
    let dir = args.dir;
    let list = args.list;

    let trash = get_trash_directory();

    // listing the trash directory if the flag is set
    if list {
        list_trash();
        return;
    }

    // tidying the trash directory if the flag is set
    if tidy {
        tidy_trash_directory();
        return;
    }

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

        if let Err(e) = move_to_trash(&path) {
            eprintln!("Error moving to trash: {e}");
        }
    }
}
