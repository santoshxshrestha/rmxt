use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "rmxd", author = "santoshxshrestha", version, about)]
pub struct Args {
    /// Path of the file (when no subcommand is used)
    #[arg(help = "Files or directories to remove")]
    pub file: Vec<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Don't put the file in trash, remove it permanently
    #[arg(short = 'i', long, global = true)]
    pub ignore: bool,

    /// Purge files from the trash directory
    #[arg(short = 'p', long, global = true)]
    pub purge: bool,

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

    /// Recover files from the trash directory
    #[command(name = "recover")]
    Recover {
        /// Name of the file to recover
        #[arg(help = "Name of the file to recover from trash")]
        name: String,
    },
}

impl Args {
    /// Get the files to remove, handling the default case
    pub fn get_files(&self) -> Vec<PathBuf> {
        match &self.command {
            Some(_) => Vec::new(),     // No files for list/tidy/recover commands
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

    /// Check if recover command is active
    pub fn is_recover(&self) -> bool {
        matches!(self.command, Some(Commands::Recover { .. }))
    }

    /// Get the name to recover (if recover command is active)
    pub fn get_recover_name(&self) -> Option<&str> {
        match &self.command {
            Some(Commands::Recover { name }) => Some(name),
            _ => None,
        }
    }

    /// Check if remove command is active (default behavior when no subcommand)
    pub fn is_remove(&self) -> bool {
        self.command.is_none()
    }

    /// Check if purge flag is set
    pub fn is_purge(&self) -> bool {
        self.purge
    }
}
