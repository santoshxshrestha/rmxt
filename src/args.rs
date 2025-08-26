use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "rmxd",
    author = "santoshxshrestha",
    version,
    about = "Remove files or directories"
)]
pub struct Args {
    /// Path of the file
    pub file: Vec<PathBuf>,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', long, default_value = "true")]
    pub recursive: bool,
}
