# rmxt

A safer, recoverable alternative to the traditional `rm` command that moves files to the system trash instead of permanently deleting them.

## Features

- **Cross-platform trash support** - Works on Linux, macOS, and Windows
- **File recovery** - Restore individual files, files from specific day to now or all files from trash
- **Trash management** - List, purge, and automatically clean old files
- **Shell integration** - Drop-in replacement for `rm` command
- **Safety first** - Prevents accidental permanent deletion
- **Flexible options** - Force, recursive, and bypass modes available
- **Enhanced output** - Colored error messages and formatted table display for trash listings
- **Time-based operations** - Filter and manage files based on deletion timestamps

## Installation

### From crates.io

```bash
cargo install rmxt
```

### From Source

```bash
git clone https://github.com/santoshxshrestha/rmxt
cd rmxt
cargo build --release
sudo cp target/release/rmxt /usr/local/bin/
```

## Commands & Usage

### Basic File Operations

```bash
# Remove files (move to trash)
rmxt file.txt
rmxt file1.txt file2.txt file3.txt
rmxt *.log

# Remove directories recursively
rmxt -r directory/

# Remove empty directories
rmxt -d empty_directory/

# Force removal without prompts
rmxt -f file.txt

# Combined options
rmxt -rf directory/          # Recursive + force
rmxt -df empty_dir1/ empty_dir2/  # Directory + force
```

### Trash Management

```bash
# List all files in trash with details
rmxt list

# List all files in trash from the last 10 days
rmxt list -t 10

# Recover specific file from trash
rmxt recover filename.txt

# Recover all files from trash to their original locations
rmxt recover-all

# Recover all files from 20 days ago to now
rmxt recover-all -t 20

# Permanently delete specific file from trash
rmxt purge filename.txt

# Clean trash (remove files older than 30 days)
rmxt tidy

# Clean trash (remove files older then 20 days)
rmxt tidy -t 20
```

### Permanent Deletion (Bypass Trash)

```bash
# Permanently delete without using trash
rmxt -i file.txt
rmxt -i file1.txt file2.txt

# Permanently delete directory
rmxt -ir directory/

# Permanently delete with force
rmxt -ifr directory/
```

> **⚠️ Warning:** The `-i, --ignore` flag permanently deletes files without moving them to trash. Use with caution!

## Command Reference

### All Flags

| Flag | Long Form     | Description                                 |
| ---- | ------------- | ------------------------------------------- |
| `-i` | `--ignore`    | Permanently delete without using trash      |
| `-r` | `--recursive` | Remove directories and contents recursively |
| `-f` | `--force`     | Force removal without prompts               |
| `-d` | `--dir`       | Remove empty directories                    |
| `-h` | `--help`      | Show help information                       |
| `-V` | `--version`   | Show version information                    |
| `-t` | `--time`      | Specify days for recovery or tidy commands  |

### Subcommands

| Command          | Description                                                         |
| ---------------- | ------------------------------------------------------------------- |
| `list`           | Show all files in trash with deletion timestamps and original paths in a formatted table |
| `recover <name>` | Restore specific file from trash to its original location           |
| `recover-all`    | Restore all files from trash to their original locations            |
| `purge <name>`   | Permanently delete specific file from trash                         |
| `tidy`           | Permanently delete files older than 30 days from trash              |
| `help`           | Show help message or help for specific subcommand                   |

## Trash Location

Files are moved to the system trash directory using platform-native locations:

- **Linux**: `~/.local/share/Trash/files/`
- **macOS**: `~/.Trash/`
- **Windows**: Recycle Bin

The exact location is managed by the system's trash implementation, ensuring compatibility with your desktop environment's trash functionality.

## File Recovery

### Using Commands

```bash
# List what's in trash
rmxt list

# Sample output (formatted table):
# ┌──────────────┬─────────────────────────┬─────────────────────┐
# │ name         │ original_location       │ deleted_at          │
# ├──────────────┼─────────────────────────┼─────────────────────┤
# │ document.pdf │ /home/user/Documents    │ 2024-01-15 14:30:22 │
# │ image.png    │ /home/user/Pictures     │ 2024-01-14 09:15:10 │
# └──────────────┴─────────────────────────┴─────────────────────┘

# Recover specific file
rmxt recover document.pdf

# Recover all files
rmxt recover-all
```

### Important Recovery Notes

- Files are restored to their original locations when possible
- Original file permissions and timestamps are preserved
- If the original directory no longer exists, recovery may fail
- Use `rmxt list` to see available files and their original paths

## Shell Integration

Replace `rm` with `rmxt` by adding aliases to your shell configuration:

### Bash/Zsh

Add to `~/.bashrc` or `~/.zshrc`:

```bash
alias rm='rmxt'
```

### Fish Shell

Add to `~/.config/fish/config.fish`:

```fish
alias rm='rmxt'
```

### PowerShell (Windows)

Add to your PowerShell profile:

```powershell
Set-Alias rm rmxt
```

After adding aliases, reload your shell:

```bash
# Bash/Zsh
source ~/.bashrc  # or ~/.zshrc

# Fish
source ~/.config/fish/config.fish
```

## Dependencies

This project uses the following key dependencies:

- **[chrono](https://crates.io/crates/chrono)** - Date and time handling for trash cleanup
- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing with derive macros
- **[trash](https://crates.io/crates/trash)** - Cross-platform system trash integration
- **[walkdir](https://crates.io/crates/walkdir)** - Recursive directory traversal
- **[dirs](https://crates.io/crates/dirs)** - Platform-specific directory utilities
- **[colored](https://crates.io/crates/colored)** - Terminal text coloring for error messages
- **[tabled](https://crates.io/crates/tabled)** - Table formatting for trash listing output

## Development Status & Improvements

### Recent Updates (v0.1.7)

- **Enhanced user interface** - Error messages now display in red color for better visibility
- **Improved output formatting** - Trash listings now show in a clean, formatted table
- **Better error handling** - More robust error handling throughout the application
- **Time-based filtering** - All commands now support time-based filtering options
- **Multiple file support** - `recover` and `purge` commands can now handle multiple files at once
- **Rust 2024 edition** - Updated to use the latest Rust edition for improved performance

### Current Limitations

- Some functions still use `unwrap()` for error handling, which may cause panics on unexpected errors
- Limited graceful error recovery in some edge cases

### Planned Improvements

- Continue replacing `unwrap()` calls with proper error propagation using `Result` and `?` operator
- Enhanced configuration options for trash behavior
- More robust file conflict resolution
- Performance optimizations for large trash directories

## Contributing

Contributions are welcome! Please feel free to:

- Report bugs and issues
- Suggest new features
- Submit pull requests
- Improve documentation

## License

This project is licensed under the [MIT License](LICENSE).

## Repository

- **Homepage**: https://github.com/santoshxshrestha/rmxt
- **Documentation**: https://github.com/santoshxshrestha/rmxt#readme
- **Issues**: https://github.com/santoshxshrestha/rmxt/issues
