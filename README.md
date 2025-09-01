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

## 📚 Documentation

For comprehensive guides and detailed examples, see our modular documentation:

- **[Installation Guide](docs/installation.md)** - Detailed installation instructions for all platforms
- **[Usage Guide](docs/usage.md)** - Comprehensive usage examples and workflows  
- **[Shell Integration](docs/shell-integration.md)** - Complete shell setup and alias configuration
- **[Advanced Features](docs/advanced-features.md)** - Power user features and automation
- **[Troubleshooting](docs/troubleshooting.md)** - Solutions for common issues and problems

## Quick Start

### Basic Installation

```bash
cargo install rmxt
```

For detailed installation instructions including platform-specific setup, see the **[Installation Guide](docs/installation.md)**.

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

For complete shell integration including advanced configurations, tab completion, and cross-shell compatibility, see the **[Shell Integration Guide](docs/shell-integration.md)**.

## Command Reference (Quick Reference)

### Flags

| Flag | Long Form     | Description                                 |
| ---- | ------------- | ------------------------------------------- |
| `-i` | `--ignore`    | Permanently delete without using trash      |
| `-r` | `--recursive` | Remove directories and contents recursively |
| `-f` | `--force`     | Force removal without prompts               |
| `-d` | `--dir`       | Remove empty directories                    |
| `-t` | `--time`      | Specify days for recovery or tidy commands  |

### Commands

| Command          | Description                                      |
| ---------------- | ------------------------------------------------ |
| `list [-t days]` | Show files in trash with optional time filter   |
| `recover <name>` | Restore specific file from trash                |
| `recover-all`    | Restore all files from trash                    |
| `purge <name>`   | Permanently delete specific file from trash     |
| `tidy [-t days]` | Remove old files from trash (default: 30 days)  |

> **⚠️ Warning:** The `-i, --ignore` flag permanently deletes files without moving them to trash. Use with caution!

## Advanced Features

For power users and complex workflows:

- **[Scripting and Automation](docs/advanced-features.md#scripting-and-automation)** - Integrate rmxt into build systems and scripts
- **[Cross-Platform Integration](docs/advanced-features.md#cross-platform-trash-integration)** - Platform-specific optimizations
- **[Performance Optimization](docs/advanced-features.md#performance-optimization)** - Handle large directories efficiently  
- **[Integration with Other Tools](docs/advanced-features.md#integration-with-other-tools)** - Use with find, fd, ripgrep, and git

## Troubleshooting

Having issues? Check the **[Troubleshooting Guide](docs/troubleshooting.md)** for solutions to common problems:

- Installation and compilation issues
- Permission and access problems  
- Recovery and trash system issues
- Platform-specific problems
- Performance and integration issues

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

### Recent Updates (v0.1.7)

- Enhanced user interface with colored error messages
- Improved output formatting with clean table display
- Better error handling throughout the application
- Time-based filtering for all commands
- Multiple file support for recover and purge commands
- Updated to Rust 2024 edition

### Planned Improvements

- Replace `unwrap()` calls with proper error propagation
- Enhanced configuration options for trash behavior
- More robust file conflict resolution
- Performance optimizations for large directories

For technical details and advanced development topics, see the documentation guides.

## License

This project is licensed under the [MIT License](LICENSE).

## Repository

- **Homepage**: https://github.com/santoshxshrestha/rmxt
- **Documentation**: https://github.com/santoshxshrestha/rmxt#readme
- **Issues**: https://github.com/santoshxshrestha/rmxt/issues
