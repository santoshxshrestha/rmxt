mod args;
use args::Args;
use clap::Parser;
use dirs::home_dir;
use std::fs::{self, rename};

fn main() {
    // parsing the args
    let args = Args::parse();

    let paths = args.file;
    let recursive = args.recursive;
    let force = args.force;
    let tidy = args.tidy;
    let dir = args.dir;

    // getting the home directory and appending .trash to it
    let trash = home_dir().unwrap().join(".trash/");

    // tidying the trash directory if the flag is set
    if tidy {
        fs::remove_dir_all(&trash).unwrap();
    }

    // creating the trash directory if it doesn't exist
    if !fs::exists(&trash).unwrap() {
        fs::create_dir(&trash).unwrap();
    }

    // iterating over the paths
    for path in paths {
        if path.is_dir() && !recursive {
            if !force {
                eprintln!("rmxd: cannot remove {path:?}: Is a directory");
            }
            if dir && fs::read(&path).iter().next().is_none() {
                fs::remove_dir(&path).unwrap()
            }
            continue;
        }

        // moving the file to trash
        let name_of_file = path.file_name().unwrap();
        let new_path = trash.join(name_of_file);

        if let Err(e) = rename(path, new_path) {
            if !force {
                eprintln!("Error moving file to trash: {e}");
            }
        }
    }
}
