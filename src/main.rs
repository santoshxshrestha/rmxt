#![allow(unused)]
mod args;
use args::Args;
use clap::{Parser, parser};

fn main() {
    let args = Args::parse();

    println!("Processing file: {}", args.file);
}
