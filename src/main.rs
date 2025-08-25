#![allow(unused)]
mod args;
use args::Args;
use clap::{Parser, parser};

fn main() {
    let args = Args::parse();

    match args {
        Args { file: f } => {
            println!("File to be removed: {}", f);
        }
    }
}
