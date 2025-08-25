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
    pub file: PathBuf,
}
