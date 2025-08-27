use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "rmxd",
    author = "santoshxshrestha",
    version,
    about = "Remove files or directories"
)]
pub struct Args {
    /// Path of the file (when no subcommand is used)
    #[arg(help = "Files or directories to remove")]
    pub file: Vec<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', long, global = true)]
    pub recursive: bool,

    /// Force removal of files without prompt
    #[arg(short = 'f', long, global = true)]
    pub force: bool,

    /// Remove empty directories
    #[arg(short = 'd', long, global = true)]
    pub dir: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List files in the trash directory
    #[command(name = "list")]
    List,

    /// Clean up the trash directory
    #[command(name = "tidy")]
    Tidy,
}

impl Args {
    /// Get the files to remove, handling the default case
    pub fn get_files(&self) -> Vec<PathBuf> {
        match &self.command {
            Some(_) => Vec::new(),     // No files for list/tidy commands
            None => self.file.clone(), // Use the default file argument
        }
    }

    /// Check if list command is active
    pub fn is_list(&self) -> bool {
        matches!(self.command, Some(Commands::List))
    }

    /// Check if tidy command is active
    pub fn is_tidy(&self) -> bool {
        matches!(self.command, Some(Commands::Tidy))
    }

    /// Check if remove command is active (default behavior when no subcommand)
    pub fn is_remove(&self) -> bool {
        self.command.is_none()
    }
}
