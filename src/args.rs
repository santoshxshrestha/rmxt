use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "rmxd",
    author = "santoshxshrestha",
    version,
    about = "Remove files or directories"
)]
pub struct Args {
    /// Path of the file
    pub file: String,
}

