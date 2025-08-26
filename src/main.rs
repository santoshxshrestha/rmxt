#![allow(unused)]
mod args;
use args::Args;
use clap::{Parser, parser};
use dirs::home_dir;
use std::fs::File;
use std::fs::remove_file;
use std::fs::{self, rename};
use std::path::Path;
use std::path::PathBuf;

fn main() {
    // parsing the args
    let args = Args::parse();
    let paths = args.file;
    let recursive = args.recursive;

    // creating the trash directory
    let trash = home_dir().unwrap().join(".trash/");

    if !fs::exists(&trash).unwrap() {
        fs::create_dir(&trash).unwrap();
    }

    for path in paths {
        if path.is_dir() && !recursive {
            eprintln!("rmxd: cannot remove '{path:?}': Is a directory");
            return;
        }
        // moving the file to trash
        let name_of_file = path.file_name().unwrap();
        let new_path = trash.join(name_of_file);

        if let Err(e) = rename(path, new_path) {
            eprintln!("Error deleting file: {e}");
        }
    }
}
