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

    // creating the trash directory
    let trash = home_dir().unwrap().join(".trash/");

    if !fs::exists(&trash).unwrap() {
        fs::create_dir(&trash).unwrap();
    }

    for path in paths {
        if path.is_dir() && !recursive {
            if !force {
                eprintln!("rmxd: cannot remove {path:?}: Is a directory");
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
