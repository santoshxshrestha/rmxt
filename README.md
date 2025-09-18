# rmxt

A safer, recoverable alternative to the traditional `rm` command that moves files to the system trash instead of permanently deleting them.

## Features

- **File recovery** - Restore individual files, files from specific day to now or all files from trash
- **Trash management** - List, purge, and automatically clean old files
- **Shell integration** - Drop-in replacement for `rm` command
- **Safety first** - Prevents accidental permanent deletion
- **Flexible options** - Force, recursive, and bypass modes available
- **Enhanced output** - Colored error messages and formatted table display for trash listings
- **Time-based operations** - Filter and manage files based on deletion timestamps
- **Name conflict resolution** - Automatically appends the current date and time as a suffix if a file with the same name already exists in the trash

## Documentation

For comprehensive guides and detailed examples, see our modular documentation:

- **[Installation Guide](docs/install.md)** - Detailed installation instructions for all platforms
- **[Usage Guide](docs/usage.md)** - Comprehensive usage examples and workflows
- **[Advanced Features](docs/advanced.md)** - Shell integration, automation, and troubleshooting

## Quick Start

### Installation Script (Recommended)

The fastest way to install rmxt:

```bash
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/install.sh | bash
```

### Basic Installation

```bash
cargo install rmxt
```

### Nix Flake (for Nix users)

If you use Nix, you can run or build `rmxt` without installing Rust:

```bash
nix run github:santoshxshrestha/rmxt
```

Or build the binary:

```bash
nix build github:santoshxshrestha/rmxt
```

See the **[Installation Guide](docs/install.md)** for full details and prerequisites.

For detailed installation instructions including platform-specific setup, see the **[Installation Guide](docs/install.md)**.

## Basic Usage

```bash
# Remove files safely (move to trash)
rmxt file.txt directory/

# List files in trash
rmxt list

# Recover files from trash
rmxt recover file.txt
rmxt recover-all

# Clean old files from trash
rmxt tidy
```

For comprehensive usage examples, workflows, and advanced operations, see the **[Usage Guide](docs/usage.md)**.

## Shell Integration

Replace `rm` with `rmxt` for safer file operations:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias rm='rmxt'
```

For complete shell integration including advanced configurations, tab completion, and cross-shell compatibility, see the **[Advanced Features Guide](docs/advanced.md)**.

## Command Reference (Quick Reference)

### Flags

| Flag | Long Form     | Description                                 |
| ---- | ------------- | ------------------------------------------- |
| `-p` | `--permanent` | Permanently delete without using trash      |
| `-r` | `--recursive` | Remove directories and contents recursively |
| `-f` | `--force`     | Force removal without prompts               |
| `-d` | `--dir`       | Remove empty directories                    |
| `-t` | `--time`      | Specify days for recovery or tidy commands  |

### Commands

| Command          | Description                                    |
| ---------------- | ---------------------------------------------- |
| `list [-t days]` | Show files in trash with optional time filter  |
| `recover <name>` | Restore specific file from trash               |
| `recover-all`    | Restore all files from trash                   |
| `purge <name>`   | Permanently delete specific file from trash    |
| `tidy [-t days]` | Remove old files from trash (default: 30 days) |

> **⚠️ Warning:** The `-p, --permanent` flag permanently deletes files without moving them to trash. Use with caution!

## Advanced Features

For power users and complex workflows:

- **[Shell Integration and Automation](docs/advanced.md#shell-integration)** - Complete shell setup and automation
- **[Troubleshooting](docs/advanced.md#troubleshooting)** - Solutions for common problems

## Troubleshooting

Having issues? Check the **[Advanced Features Guide](docs/advanced.md#troubleshooting)** for solutions to common problems:

- Installation and compilation issues
- Permission and access problems
- Recovery and trash system issues

## Cross-Platform Support

rmxt integrates with native system trash implementations across platforms:

- **Linux**: `~/.local/share/Trash/files/` (XDG specification)
- **macOS**: `~/.Trash/` (Finder integration)
- **Windows**: Recycle Bin (native integration)

The exact location is managed by the system's trash implementation, ensuring compatibility with your desktop environment.

## Development & Contributing

### How to Contribute

We welcome contributions! Here's how you can help:

1. **Report Issues**: Found a bug or have a feature request? [Open an issue](https://github.com/santoshxshrestha/rmxt/issues)
2. **Submit Pull Requests**: Fix bugs or add features with a pull request
3. **Improve Documentation**: Help expand or clarify the documentation
4. **Share Feedback**: Let us know how you use rmxt and what could be better

### Development Setup

```bash
# Clone the repository
git clone https://github.com/santoshxshrestha/rmxt
cd rmxt

# Build and test
cargo build
cargo test

# Install locally for testing
cargo install --path .
```
---
### Planned Improvements

- Enhanced configuration options for trash behavior
- Performance optimizations for large directories

For technical details and advanced development topics, see the documentation guides.

## License

This project is licensed under the [MIT License](LICENSE).

## Repository

- **Homepage**: https://github.com/santoshxshrestha/rmxt
- **Documentation**: https://github.com/santoshxshrestha/rmxt#readme
- **Issues**: https://github.com/santoshxshrestha/rmxt/issues
