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
    #[arg(short = 'i', long, global = false)]
    pub ignore: bool,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', long, global = false)]
    pub recursive: bool,

    /// Force removal of files without prompt
    #[arg(short = 'f', long, global = false)]
    pub force: bool,

    /// Remove empty directories
    #[arg(short = 'd', long, global = false)]
    pub dir: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List files in the trash directory
    #[command(name = "list")]
    List,

    /// Clean up the trash directory by removing files older than 30 days
    #[command(name = "tidy")]
    Tidy {
        /// Specify time to live for tidy command (in days) (default is 30 days)
        #[arg(short = 't', long, global = false)]
        time: Option<i64>,
    },

    /// recover all the content of the trash
    #[command(name = "recover-all")]
    RecoverAll {
        /// Specify time from which to recover files (in days)
        #[arg(short = 't', long, global = false)]
        time: Option<i64>,
    },

    /// Recover files from the trash directory
    #[command(name = "recover")]
    Recover {
        /// Name of the file to recover
        #[arg(help = "Name of the file to recover from trash")]
        name: String,
    },

    /// Purge files from the trash directory
    #[command(name = "purge")]
    Purge {
        /// Purge files from the trash directory
        #[arg(help = "Name of the file to purge")]
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
        matches!(self.command, Some(Commands::Tidy { time }))
    }

    /// Check if RecoverAll command is active
    pub fn is_recover_all(&self) -> bool {
        matches!(self.command, Some(Commands::RecoverAll { time }))
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

    /// Check if purge flag is set
    pub fn is_purge(&self) -> bool {
        matches!(self.command, Some(Commands::Purge { .. }))
    }

    /// Get the name to purge (if purge command is active)
    pub fn get_purge_name(&self) -> Option<&str> {
        match &self.command {
            Some(Commands::Purge { name }) => Some(name),
            _ => None,
        }
    }

    /// Check if remove command is active (default behavior when no subcommand)
    pub fn is_remove(&self) -> bool {
        self.command.is_none()
    }

    /// Get the time to live for tidy command
    pub fn get_time_tidy(&self) -> i64 {
        match &self.command {
            Some(Commands::Tidy { time }) => time.unwrap_or(30), // Default to 30 days if not specified
            _ => 30, // Default to 30 days if not tidy command
        }
    }

    /// Get the time from which to recover files for RecoverAll command
    pub fn get_time_recover(&self) -> i64 {
        match &self.command {
            // Default to 0 days (which will be evaluated to all the content)if not specified
            Some(Commands::RecoverAll { time }) => time.unwrap_or(0),
            _ => 0, // Default to 0 days if not RecoverAll command
        }
    }
}
