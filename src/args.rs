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

    /// List files in the trash directory
    #[arg(long, default_value = "false")]
    pub list: bool,

    /// Clean up the trash directory
    #[arg(long, default_value = "false")]
    pub tidy: bool,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', long, default_value = "false")]
    pub recursive: bool,

    /// Force removal of files without prompt
    #[arg(short = 'f', long, default_value = "false")]
    pub force: bool,

    /// Remove empty directories
    #[arg(short = 'd', long, default_value = "false")]
    pub dir: bool,
}
