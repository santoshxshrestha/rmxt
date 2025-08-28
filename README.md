# rmxt

`rmxt` is a safer, recoverable alternative to the traditional `rm` command. Instead of permanently deleting files, `rmxt` moves them to a trash directory, allowing you to recover them later if needed.

> **Note:** We do not move empty directories to the trash directory because they serve no purpose and can be recreated easily if needed.

## Features

- Prevents accidental permanent deletion of files.
- Moves deleted files to a designated trash directory.
- List and manage files in the trash directory.
- Clean up the trash directory when needed.

## Installation

### Using Cargo Package Manager

If you have Rust and Cargo installed, you can install `rmxt` directly from crates.io:

```bash
cargo install rmxt
```

### Build and Install from Source

Alternatively, you can build and install from source:

```bash
cargo build --release
sudo cp target/release/rmxt /usr/local/bin/
```

## Setting up Shell Aliases

To use `rmxt` as a replacement for the traditional `rm` command, you can set up aliases in your shell configuration:

### Bash
Add this line to your `~/.bashrc` or `~/.bash_profile`:
```bash
alias rm='rmxt'
```

### Zsh
Add this line to your `~/.zshrc`:
```zsh
alias rm='rmxt'
```

### Fish
Add this command to your Fish configuration:
```fish
alias rm='rmxt'
```

After adding the alias, reload your shell configuration:
- Bash/Zsh: `source ~/.bashrc` (or `~/.zshrc`)
- Fish: `source ~/.config/fish/config.fish`

## Usage Examples

### Basic File Removal
```bash
# Remove a single file
rmxt file.txt

# Remove multiple files
rmxt file1.txt file2.txt file3.txt

# Remove files with patterns (if your shell supports globbing)
rmxt *.log
```

### Directory Operations
```bash
# Remove a directory and its contents recursively
rmxt -r directory/

# Remove an empty directory
rmxt -d empty_directory/

# Force remove without prompts
rmxt -f file.txt
```

### Managing the Trash Directory
```bash
# List all files in the trash
rmxt list

# Clean up the trash directory (permanently delete all trashed files)
rmxt tidy
```

### Permanent Deletion (Bypass Trash)
```bash
# Permanently delete a file (bypass trash directory)
rmxt -i file.txt

# Permanently delete multiple files
rmxt -i file1.txt file2.txt

# Permanently delete a directory and its contents
rmxt -ir directory/
```

> **⚠️ Warning:** When using the `-i, --ignore` option, files are permanently deleted and cannot be recovered from the trash directory. Use with caution!

### Combined Options
```bash
# Recursively and forcefully remove a directory
rmxt -rf directory/

# Remove empty directories with force
rmxt -df empty_dir1/ empty_dir2/

# Permanently and forcefully remove a directory
rmxt -ifr directory/
```

## Trash Directory Location

### Default Location
The trash directory is located at:
```
~/.trash/
```
Where `~` represents your home directory (e.g., `/home/username/.trash/` on Linux, `/Users/username/.trash/` on macOS).

### Directory Structure
```
~/.trash/
├── file1.txt
├── document.pdf
├── folder1/
│   ├── nested_file.txt
│   └── subfolder/
└── script.sh
```

## Manual Recovery

### Recovering Individual Files
You can manually recover files from the trash directory:

```bash
# Navigate to the trash directory
cd ~/.trash/

# List all trashed files
ls -la

# Move a file back to your desired location
mv file.txt ~/Documents/

# Move a directory back
mv folder1/ ~/Projects/
```

### Recovering with Original Structure
Since `rmxt` moves files directly to `~/.trash/`, files lose their original directory structure. To help identify files:

```bash
# List files with details (timestamps can help identify recent deletions)
ls -lat ~/.trash/

# Find files by name
find ~/.trash/ -name "*.txt" -type f

# Find directories
find ~/.trash/ -type d
```

### Important Recovery Notes
- Files in the trash retain their original names but lose their directory path
- If multiple files with the same name are deleted, `rmxt` prevents overwriting by automatically appending a number to the filename (e.g., `file.txt`, `file.txt.1`, `file.txt.2`)
- Use `rmxt list` to see all trashed items
- Recovered files will have their original permissions intact

## Command Line Options

| Option | Description |
|--------|-------------|
| `-i, --ignore` | Don't put the file in trash, remove it permanently |
| `-r, --recursive` | Remove directories and their contents recursively |
| `-f, --force` | Force removal without prompts |
| `-d, --dir` | Remove empty directories |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

### Subcommands

| Command | Description |
|---------|-------------|
| `list` | List all files in the trash directory |
| `tidy` | Permanently delete all files in the trash directory |
| `help` | Print help message or help for specific subcommand |

## Warning

The current implementation of `rmxt` relies heavily on the use of `unwrap()` for error handling. This means:

- If any operation (e.g., file I/O, directory creation) fails, the program will panic and terminate abruptly.
- There is no graceful recovery or fallback mechanism in place for unexpected errors.

However, this is a work in progress, and the logic will be refactored to convert the code into a safer implementation. Future updates will:

- Replace `unwrap()` with proper error propagation using `Result` and the `?` operator.
- Introduce robust error handling to ensure the program can recover gracefully from unexpected failures.

## License

This project is licensed under the [MIT License](LICENSE).
