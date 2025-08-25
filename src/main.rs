#![allow(unused)]
mod args;
use args::Args;
use clap::{Parser, parser};
use std::fs::remove_file;

fn main() {
    let args = Args::parse();
    let file_path = args.file;
    if let Err(e) = remove_file(file_path) {
        eprintln!("Error deleting file: {e}");
    }
}
