# Advanced Usage

## Shell Integration

### Replace `rm` Command

```bash
# Add to ~/.bashrc, ~/.zshrc, or ~/.config/fish/config.fish
alias rm='rmxt'
```

### Safer Wrapper Function

```bash
# Bash/Zsh
function rm() {
    if [[ "$*" == *"-i"* ]]; then
        echo "Warning: Permanent deletion!"
        read -p "Continue? (y/N) " -n 1 -r
        echo
        [[ $REPLY =~ ^[Yy]$ ]] && rmxt "$@"
    else
        rmxt "$@"
    fi
}
```

## Automation

### Cleanup Scripts

```bash
#!/bin/bash
# daily-cleanup.sh
rmxt -rf build/ dist/ target/
rmxt *.log *.tmp *.cache
rmxt tidy -t 30
```

### Find Integration

```bash
# Remove old log files
find . -name "*.log" -mtime +7 -exec rmxt {} \;

# With fd (faster)
fd -e log -x rmxt
```

## Troubleshooting

### Command Not Found

```bash
# Check PATH
echo $PATH | tr ':' '\n' | grep -E '(cargo|local)'

# Add cargo bin to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```

### Permission Issues

```bash
# User install instead of system
cp target/release/rmxt ~/.local/bin/
```

### Trash Problems

```bash
# Check trash location (varies by OS)
# Linux: ~/.local/share/Trash/
# macOS: ~/.Trash/
# Windows: Recycle Bin

# Reset if corrupted
rmxt recover-all
rmxt tidy -t 0  # Clean all
```

### Recovery Issues

```bash
# List with full paths
rmxt list

# File name must match exactly
rmxt recover "file with spaces.txt"
```

## Configuration

No config file needed - rmxt works out of the box. Behavior is controlled through command flags and environment variables.

### Environment Variables

```bash
export RMXT_DEFAULT_DAYS=7    # Default tidy days
export RMXT_CONFIRM=false     # Skip confirmations
```