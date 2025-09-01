# Troubleshooting Guide

This comprehensive troubleshooting guide helps you resolve common issues with `rmxt` and provides solutions for edge cases across different platforms.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Command Not Found](#command-not-found)
3. [Permission Problems](#permission-problems)
4. [Trash System Issues](#trash-system-issues)
5. [Recovery Problems](#recovery-problems)
6. [Performance Issues](#performance-issues)
7. [Platform-Specific Issues](#platform-specific-issues)
8. [Integration Problems](#integration-problems)
9. [Data Corruption and Safety](#data-corruption-and-safety)
10. [Debugging and Diagnostics](#debugging-and-diagnostics)

## Installation Issues

### Cargo Install Fails

**Problem:** `cargo install rmxt` fails with compilation errors.

**Solutions:**

```bash
# Update Rust toolchain
rustup update

# Clear cargo cache and retry
cargo cache --clean
cargo install rmxt

# Install with verbose output to see detailed errors
cargo install rmxt --verbose

# Force reinstall if partially installed
cargo install rmxt --force
```

**Common error patterns:**

```bash
# Linker errors on Linux
sudo apt-get install build-essential pkg-config libssl-dev

# macOS compilation issues  
xcode-select --install

# Windows compilation issues (using rustup)
rustup target add x86_64-pc-windows-msvc
```

### Network/Proxy Issues

**Problem:** Cannot download crates due to network restrictions.

**Solutions:**

```bash
# Configure cargo to use proxy
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[http]
proxy = "http://proxy.company.com:8080"

[https] 
proxy = "http://proxy.company.com:8080"
EOF

# Use alternative registry if crates.io is blocked
[source.crates-io]
replace-with = "company-registry"

[source.company-registry]
registry = "https://company-crates.example.com"
```

### Version Compatibility

**Problem:** Rust version too old for rmxt compilation.

**Solutions:**

```bash
# Check current Rust version
rustc --version

# Update to latest stable
rustup update stable
rustup default stable

# Install specific version if needed
rustup install 1.70.0
rustup default 1.70.0
```

## Command Not Found

### PATH Issues

**Problem:** `rmxt: command not found` after successful installation.

**Diagnostic:**

```bash
# Check if binary exists
ls ~/.cargo/bin/rmxt
which rmxt
echo $PATH | grep -o ~/.cargo/bin
```

**Solutions:**

```bash
# Add cargo bin to PATH (bash/zsh)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For fish shell
set -U fish_user_paths $HOME/.cargo/bin $fish_user_paths

# For PowerShell (Windows)
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

### Shell Alias Conflicts

**Problem:** `rm` alias not working or conflicting.

**Diagnostic:**

```bash
# Check current aliases
alias | grep rm
type rm
which rm
```

**Solutions:**

```bash
# Remove conflicting aliases
unalias rm

# Check for functions overriding alias
unset -f rm

# Verify rmxt is accessible
/home/user/.cargo/bin/rmxt --version

# Re-create alias
alias rm='rmxt'
```

### Multiple Installations

**Problem:** Multiple versions or installations causing conflicts.

**Diagnostic:**

```bash
# Find all rmxt binaries
find /usr -name "rmxt" 2>/dev/null
find /home -name "rmxt" 2>/dev/null
whereis rmxt
```

**Solutions:**

```bash
# Remove old installations
sudo rm /usr/local/bin/rmxt
rm ~/.local/bin/rmxt

# Reinstall from cargo
cargo install rmxt --force

# Verify single installation
which rmxt
rmxt --version
```

## Permission Problems

### Trash Directory Access

**Problem:** Permission denied when accessing trash directory.

**Diagnostic:**

```bash
# Check trash directory permissions
ls -la ~/.local/share/Trash/
ls -la ~/.Trash/  # macOS

# Check ownership
stat ~/.local/share/Trash/files/
```

**Solutions:**

```bash
# Fix ownership (Linux)
sudo chown -R $USER:$USER ~/.local/share/Trash/
chmod 755 ~/.local/share/Trash/
chmod 755 ~/.local/share/Trash/files/
chmod 755 ~/.local/share/Trash/info/

# Recreate trash directory if corrupted
rm -rf ~/.local/share/Trash/
mkdir -p ~/.local/share/Trash/{files,info}
```

### System Directory Removal

**Problem:** Cannot remove files from system directories.

**Diagnostic:**

```bash
# Check file permissions
ls -la /path/to/file
lsattr /path/to/file  # Linux extended attributes
```

**Solutions:**

```bash
# Use sudo for system files (be very careful)
sudo rmxt /var/log/old.log

# Change ownership first (if appropriate)
sudo chown $USER:$USER /path/to/file
rmxt /path/to/file

# Remove immutable attributes (Linux)
sudo chattr -i /path/to/file
rmxt /path/to/file
```

### SELinux/AppArmor Issues

**Problem:** Security policies preventing trash operations.

**Diagnostic:**

```bash
# Check SELinux status
sestatus
getenforce

# Check for denials
ausearch -m avc -ts recent | grep rmxt
```

**Solutions:**

```bash
# Temporary SELinux permissive mode (testing only)
sudo setenforce 0

# Create custom policy (consult security team)
audit2allow -a -M rmxt_custom
sudo semodule -i rmxt_custom.pp

# AppArmor profile adjustment
sudo aa-complain /usr/bin/rmxt
```

## Trash System Issues

### Trash Directory Corruption

**Problem:** Trash directory is corrupted or inconsistent.

**Symptoms:**
- `rmxt list` shows errors
- Recovery fails unexpectedly
- Trash appears empty but disk space not freed

**Solutions:**

```bash
# Backup existing trash
cp -r ~/.local/share/Trash/ ~/.local/share/Trash.backup/

# Reset trash directory
rm -rf ~/.local/share/Trash/
mkdir -p ~/.local/share/Trash/{files,info}

# Restore files manually if needed
ls ~/.local/share/Trash.backup/files/
# Manually copy important files back
```

### Cross-Platform Trash Issues

**Problem:** Trash behavior differs between platforms.

**Linux-specific:**
```bash
# Check XDG trash specification compliance
ls ~/.local/share/Trash/info/
file ~/.local/share/Trash/info/*.trashinfo

# Verify desktop environment integration
echo $XDG_CURRENT_DESKTOP
```

**macOS-specific:**
```bash
# Check Finder integration
osascript -e 'tell application "Finder" to empty trash'

# Verify .Trash permissions
ls -la ~/.Trash/
```

**Windows-specific:**
```powershell
# Check Recycle Bin access
Get-ChildItem -Path 'shell:RecycleBinFolder' -Force

# Reset Recycle Bin if corrupted
rd /s /q C:\$Recycle.Bin
```

### Disk Space Issues

**Problem:** Trash consuming too much disk space.

**Diagnostic:**

```bash
# Check trash size
du -sh ~/.local/share/Trash/
du -sh ~/.Trash/  # macOS

# Find largest files in trash
rmxt list | head -20
find ~/.local/share/Trash/files/ -type f -size +100M
```

**Solutions:**

```bash
# Progressive cleanup
rmxt tidy -t 90   # Remove files older than 90 days
rmxt tidy -t 60   # Remove files older than 60 days
rmxt tidy -t 30   # Remove files older than 30 days

# Manual cleanup of large files
rmxt purge large_file.mp4
rmxt purge huge_dataset.zip

# Emergency full cleanup
read -p "WARNING: This will permanently delete ALL trash contents. Continue? (yes/NO): " confirm
if [ "$confirm" = "yes" ]; then
    rm -rf ~/.local/share/Trash/files/*
    rm -rf ~/.local/share/Trash/info/*
fi
```

## Recovery Problems

### Files Not Recovering

**Problem:** `rmxt recover` fails or files not restored to original location.

**Diagnostic:**

```bash
# Check if file exists in trash
rmxt list | grep filename

# Check original path accessibility
ls -la /original/path/
ls -ld /original/path/

# Check trash metadata
cat ~/.local/share/Trash/info/filename.trashinfo
```

**Solutions:**

```bash
# Original directory no longer exists
mkdir -p /original/path/
rmxt recover filename

# Permission issues
sudo chown $USER:$USER /original/path/
rmxt recover filename

# Manual recovery
cp ~/.local/share/Trash/files/filename /new/location/
rmxt purge filename  # Clean up trash entry
```

### Partial Recovery

**Problem:** Some files recover successfully, others fail.

**Diagnostic script:**

```bash
#!/bin/bash
# recovery-diagnostic.sh

check_recovery_issues() {
    local failed_files=()
    
    while read -r file; do
        if ! rmxt recover "$file" 2>/dev/null; then
            failed_files+=("$file")
        fi
    done < <(rmxt list | awk 'NR>3 && NF>=3 {print $1}')
    
    if [ ${#failed_files[@]} -gt 0 ]; then
        echo "Failed to recover ${#failed_files[@]} files:"
        printf ' - %s\n' "${failed_files[@]}"
        
        # Analyze failure reasons
        for file in "${failed_files[@]}"; do
            echo "Analyzing $file..."
            
            # Check if original location exists
            local info_file=~/.local/share/Trash/info/${file}.trashinfo
            if [ -f "$info_file" ]; then
                local orig_path=$(grep "Path=" "$info_file" | cut -d= -f2)
                local orig_dir=$(dirname "$orig_path")
                
                if [ ! -d "$orig_dir" ]; then
                    echo "  Issue: Original directory does not exist: $orig_dir"
                    echo "  Solution: mkdir -p '$orig_dir'"
                fi
                
                if [ ! -w "$orig_dir" ]; then
                    echo "  Issue: No write permission to: $orig_dir"
                    echo "  Solution: chmod u+w '$orig_dir'"
                fi
            fi
        done
    fi
}

check_recovery_issues
```

### Metadata Corruption

**Problem:** Trash metadata files are corrupted.

**Diagnostic:**

```bash
# Check metadata file format
file ~/.local/share/Trash/info/*.trashinfo
head ~/.local/share/Trash/info/*.trashinfo
```

**Solutions:**

```bash
# Repair metadata manually
for info_file in ~/.local/share/Trash/info/*.trashinfo; do
    if ! grep -q "Path=" "$info_file"; then
        echo "Corrupted metadata: $info_file"
        # Attempt repair or remove
        rm "$info_file"
        
        # Remove corresponding file
        filename=$(basename "$info_file" .trashinfo)
        rm ~/.local/share/Trash/files/"$filename"
    fi
done
```

## Performance Issues

### Slow Operations

**Problem:** `rmxt` operations are very slow.

**Diagnostic:**

```bash
# Time operations
time rmxt large_directory/
time rmxt list
time rmxt tidy

# Check system resources
iostat 1 5
top -p $(pgrep rmxt)
```

**Solutions:**

```bash
# Process large directories in chunks
find large_directory/ -type f | head -1000 | xargs rmxt
find large_directory/ -type f | head -2000 | tail -1000 | xargs rmxt

# Optimize trash operations
# Clean trash regularly to reduce listing time
rmxt tidy -t 30

# Use parallel processing for multiple files
printf '%s\0' *.log | xargs -0 -P 4 -n 100 rmxt
```

### Memory Issues

**Problem:** rmxt consuming excessive memory.

**Diagnostic:**

```bash
# Monitor memory usage
ps aux | grep rmxt
pmap $(pgrep rmxt)
```

**Solutions:**

```bash
# Process in smaller batches
find . -name "*.tmp" -print0 | xargs -0 -n 50 rmxt

# Increase system limits if needed
ulimit -v unlimited
```

## Platform-Specific Issues

### Linux Issues

**Desktop Environment Integration:**

```bash
# GNOME/KDE trash integration
sudo apt-get install gvfs-bin  # GNOME
sudo apt-get install kde-cli-tools  # KDE

# Fix XDG compliance
export XDG_DATA_HOME=$HOME/.local/share
mkdir -p $XDG_DATA_HOME/Trash/{files,info}
```

**File System Issues:**

```bash
# Network file system problems
mount | grep nfs
# NFS may not support trash - use -i flag for permanent deletion

# Case-sensitive file system issues
ls -la | grep -i filename  # Check for case conflicts
```

### macOS Issues

**Finder Integration:**

```bash
# Reset Finder if trash not showing correctly
killall Finder

# Fix .Trash permissions
sudo chown $USER ~/.Trash
chmod 755 ~/.Trash
```

**SIP (System Integrity Protection) Issues:**

```bash
# Check SIP status
csrutil status

# If rmxt blocked by SIP (consult security policies)
# Temporarily disable SIP only if absolutely necessary
# Boot into Recovery Mode and run: csrutil disable
```

### Windows Issues

**Recycle Bin Integration:**

```powershell
# Reset Recycle Bin
$Shell = New-Object -ComObject Shell.Application
$RecycleBin = $Shell.Namespace(10)
$RecycleBin.Items() | ForEach-Object { $_.InvokeVerb("delete") }
```

**Path Issues:**

```powershell
# Fix long path issues
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

## Integration Problems

### Shell Alias Issues

**Problem:** Aliases not working correctly.

**Diagnostic:**

```bash
# Check shell type
echo $SHELL
echo $0

# Check alias definition
type rm
alias | grep rm
```

**Solutions:**

```bash
# Reload shell configuration
source ~/.bashrc  # bash
source ~/.zshrc   # zsh
exec $SHELL       # any shell

# Check for conflicting functions
unset -f rm
alias rm='rmxt'
```

### Script Integration Issues

**Problem:** rmxt not working in scripts.

**Solutions:**

```bash
#!/bin/bash
# Ensure PATH includes cargo bin
export PATH="$HOME/.cargo/bin:$PATH"

# Use full path in scripts
/home/user/.cargo/bin/rmxt "$@"

# Check if interactive vs non-interactive shell
if [ -t 0 ]; then
    echo "Interactive shell"
else
    echo "Non-interactive shell - aliases may not work"
fi
```

## Data Corruption and Safety

### Accidental Permanent Deletion

**Problem:** Used `-i` flag and permanently deleted important files.

**Recovery options:**

```bash
# Check if files are in system backups
sudo find /var/backups -name "*filename*" 2>/dev/null

# Check Time Machine (macOS)
tmutil list | grep filename

# Check filesystem-level recovery tools
sudo photorec  # PhotoRec for file recovery
sudo testdisk  # TestDisk for partition recovery

# Check if files exist in version control
git log --follow -- filename
```

### Preventing Future Issues

```bash
# Create safety aliases
alias rm-safe='rmxt'
alias rm-permanent='echo "WARNING: Permanent deletion"; rmxt -i'

# Create backup function
backup_before_rm() {
    local backup_dir="$HOME/.safety_backups/$(date +%Y-%m-%d)"
    mkdir -p "$backup_dir"
    cp -r "$@" "$backup_dir/" 2>/dev/null
    rmxt "$@"
}
alias rm='backup_before_rm'
```

## Debugging and Diagnostics

### Comprehensive System Check

```bash
#!/bin/bash
# rmxt-system-check.sh

echo "=== rmxt System Diagnostic ==="
echo "Date: $(date)"
echo "User: $(whoami)"
echo "OS: $(uname -a)"
echo

echo "=== Installation Check ==="
which rmxt || echo "ERROR: rmxt not found in PATH"
rmxt --version 2>/dev/null || echo "ERROR: rmxt not executable"
echo "rmxt location: $(which rmxt)"
echo

echo "=== PATH Analysis ==="
echo "PATH: $PATH"
echo "Cargo bin in PATH: $(echo $PATH | grep -o ~/.cargo/bin || echo "NO")"
echo

echo "=== Trash System Check ==="
case "$(uname -s)" in
    Linux*)
        echo "Platform: Linux"
        echo "Trash location: ~/.local/share/Trash/"
        ls -la ~/.local/share/Trash/ 2>/dev/null || echo "Trash directory not found"
        ;;
    Darwin*)
        echo "Platform: macOS"
        echo "Trash location: ~/.Trash/"
        ls -la ~/.Trash/ 2>/dev/null || echo "Trash directory not accessible"
        ;;
    *)
        echo "Platform: Other ($(uname -s))"
        ;;
esac
echo

echo "=== Current Trash Status ==="
rmxt list 2>/dev/null | head -10 || echo "ERROR: Cannot list trash contents"
echo

echo "=== Permissions Check ==="
if [ -d ~/.local/share/Trash/files ]; then
    ls -la ~/.local/share/Trash/files/ | head -5
elif [ -d ~/.Trash ]; then
    ls -la ~/.Trash/ | head -5
fi
echo

echo "=== Recent Errors ==="
# Check system logs for rmxt-related errors
case "$(uname -s)" in
    Linux*)
        journalctl --user -u "$(whoami)" | grep -i rmxt | tail -5 2>/dev/null || echo "No system logs found"
        ;;
    Darwin*)
        log show --predicate 'process == "rmxt"' --last 1h 2>/dev/null | tail -5 || echo "No system logs found"
        ;;
esac

echo "=== Diagnostic Complete ==="
```

### Performance Profiling

```bash
#!/bin/bash
# rmxt-performance-test.sh

echo "=== rmxt Performance Test ==="

# Create test files
mkdir -p test_performance
cd test_performance
for i in {1..100}; do
    echo "test content $i" > "test_file_$i.txt"
done

# Test removal performance
echo "Testing removal of 100 files..."
time rmxt *.txt

# Test listing performance
echo "Testing trash listing..."
time rmxt list > /dev/null

# Test recovery performance
echo "Testing recovery of all files..."
time rmxt recover-all > /dev/null

# Cleanup
cd ..
rm -rf test_performance
rmxt tidy -t 0  # Clean all trash

echo "Performance test complete"
```

### Debug Mode

For debugging complex issues, create a debug version:

```bash
#!/bin/bash
# rmxt-debug.sh - Wrapper with detailed logging

RMXT_DEBUG=true
RMXT_LOG_FILE="$HOME/.rmxt-debug.log"

rmxt_debug() {
    echo "$(date): $*" >> "$RMXT_LOG_FILE"
    echo "PWD: $(pwd)" >> "$RMXT_LOG_FILE"
    echo "ARGS: $*" >> "$RMXT_LOG_FILE"
    
    # Run actual rmxt and capture output
    rmxt "$@" 2>&1 | tee -a "$RMXT_LOG_FILE"
    local exit_code=${PIPESTATUS[0]}
    
    echo "EXIT_CODE: $exit_code" >> "$RMXT_LOG_FILE"
    echo "---" >> "$RMXT_LOG_FILE"
    
    return $exit_code
}

alias rmxt='rmxt_debug'
```

This troubleshooting guide provides comprehensive solutions for most issues you might encounter with `rmxt`. If you encounter issues not covered here, consider creating a GitHub issue with the diagnostic information from the system check script.