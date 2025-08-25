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
    let file_path = args.file;

    // creating the trash directory
    let trash = home_dir().unwrap().join(".trash/");

    if !fs::exists(&trash).unwrap() {
        fs::create_dir(&trash).unwrap();
    }

    // moving the file to trash
    let name_of_file = file_path.file_name().unwrap();
    let new_path = trash.join(name_of_file);

    if let Err(e) = rename(file_path, new_path) {
        eprintln!("Error deleting file: {e}");
    }
}
