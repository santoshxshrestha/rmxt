# Documentation Index

This index provides quick access to all rmxt documentation with descriptions of what each guide covers.

## Core Documentation

### [Installation Guide](installation.md) 📥
**Complete installation instructions for all platforms**
- Quick install via Cargo
- Platform-specific setup (Linux, macOS, Windows)
- Building from source
- Troubleshooting installation issues
- PATH configuration
- Updating and managing versions

### [Usage Guide](usage.md) 📖
**Comprehensive usage examples and workflows**
- Basic file operations with detailed examples
- Directory operations (empty directories, recursive removal)
- Trash management (listing, purging, cleaning)
- File recovery (individual files, bulk recovery)
- Time-based operations and filtering
- Permanent deletion and safety considerations
- Common workflows and best practices

### [Shell Integration Guide](shell-integration.md) ⚡
**Complete shell setup and customization**
- Basic alias setup for all shells (Bash, Zsh, Fish, PowerShell)
- Advanced wrapper functions with safety features
- Tab completion and auto-complete setup
- Cross-shell compatibility scripts
- Migration strategies from traditional `rm`
- Team deployment and configuration management

## Advanced Documentation

### [Advanced Features](advanced-features.md) 🚀
**Power user features and automation**
- Cross-platform trash system integration
- Advanced time management and filtering
- Scripting and automation techniques
- Integration with development tools (Make, Git, fd, ripgrep)
- Performance optimization for large operations
- Edge cases and limitations handling
- Custom configuration and environment variables

### [Troubleshooting Guide](troubleshooting.md) 🔧
**Solutions for common problems and issues**
- Installation and compilation problems
- Command not found and PATH issues
- Permission and access problems
- Trash system corruption and recovery
- Platform-specific issues (Linux, macOS, Windows)
- Performance problems and optimization
- Data safety and corruption prevention
- Comprehensive diagnostic tools and scripts

## Quick Reference

### Common Commands
```bash
# Basic operations
rmxt file.txt                    # Remove file safely
rmxt -r directory/              # Remove directory recursively
rmxt list                       # Show trash contents
rmxt recover file.txt           # Recover specific file
rmxt recover-all               # Recover all files
rmxt tidy                      # Clean old files (30+ days)

# Time-based operations
rmxt list -t 7                 # Files deleted in last 7 days
rmxt recover-all -t 1          # Recover today's deletions
rmxt tidy -t 60                # Clean files older than 60 days

# Shell integration
alias rm='rmxt'                # Replace rm with rmxt
```

### Safety Features
- **Trash Integration**: Files go to system trash, not permanent deletion
- **Recovery**: All operations are reversible unless using `-i` flag
- **Time Filtering**: Recover or clean files based on deletion time
- **Cross-Platform**: Works consistently across Linux, macOS, and Windows

## Getting Help

### First Steps
1. Start with the **[Usage Guide](usage.md)** for basic operations
2. Set up **[Shell Integration](shell-integration.md)** to replace `rm`
3. Check **[Troubleshooting](troubleshooting.md)** if you encounter issues

### For Developers
1. Read **[Advanced Features](advanced-features.md)** for automation and scripting
2. Check the **[Installation Guide](installation.md)** for development setup
3. See the main README for contributing guidelines

### Support Resources
- **GitHub Issues**: [Report bugs or request features](https://github.com/santoshxshrestha/rmxt/issues)
- **Documentation**: All guides include troubleshooting sections
- **Examples**: Every guide includes practical examples and scripts

## Document Structure

Each guide follows a consistent structure:
- **Table of Contents** - Quick navigation within each guide
- **Step-by-step Examples** - Practical, copy-pasteable commands
- **Troubleshooting Sections** - Common issues and solutions
- **Platform-specific Notes** - OS-specific variations where relevant
- **Advanced Tips** - Power user features and optimizations

## Contributing to Documentation

Found an error or want to improve the documentation?
1. Each document is a separate Markdown file in the `docs/` directory
2. Follow the existing structure and style
3. Include practical examples and code snippets
4. Test all commands and scripts before submitting
5. Submit pull requests with clear descriptions of changes

---

**Last Updated**: Current with rmxt v0.1.7  
**Maintenance**: Documentation is updated with each release