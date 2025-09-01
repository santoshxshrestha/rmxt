# Advanced Features

This guide covers advanced usage patterns, power user features, and integration techniques for `rmxt`.

## Table of Contents

1. [Cross-Platform Trash Integration](#cross-platform-trash-integration)
2. [Advanced Time Management](#advanced-time-management) 
3. [Scripting and Automation](#scripting-and-automation)
4. [Integration with Other Tools](#integration-with-other-tools)
5. [Performance Optimization](#performance-optimization)
6. [Edge Cases and Limitations](#edge-cases-and-limitations)
7. [Configuration and Customization](#configuration-and-customization)

## Cross-Platform Trash Integration

### Understanding Platform Differences

`rmxt` integrates with native system trash implementations:

**Linux:**
```bash
# Files go to XDG trash specification location
~/.local/share/Trash/files/           # Actual files
~/.local/share/Trash/info/            # Metadata files (.trashinfo)

# Check native integration
ls ~/.local/share/Trash/files/
ls ~/.local/share/Trash/info/
```

**macOS:**
```bash  
# Files go to user's Trash folder
~/.Trash/

# Verify integration with Finder
open ~/.Trash  # Opens in Finder
```

**Windows:**
```powershell
# Files go to Recycle Bin
# Location varies by drive and user
# Usually: C:\$Recycle.Bin\<SID>\
```

### Cross-Platform Scripts

Create portable scripts that work across platforms:

```bash
#!/bin/bash
# portable-cleanup.sh

# Function to check if rmxt is available
check_rmxt() {
    if ! command -v rmxt &> /dev/null; then
        echo "rmxt not found. Please install it first."
        exit 1
    fi
}

# Platform-specific cleanup
cleanup_temp() {
    case "$(uname -s)" in
        Linux*)
            rmxt /tmp/user_temp_* ~/.cache/thumbnails/
            ;;
        Darwin*)  # macOS
            rmxt ~/Library/Caches/com.*/
            rmxt /tmp/user_temp_*
            ;;
        MINGW*|CYGWIN*) # Windows
            rmxt /c/Users/$USER/AppData/Local/Temp/*
            ;;
    esac
}

check_rmxt
cleanup_temp
echo "Cross-platform cleanup completed"
```

## Advanced Time Management

### Precise Time Filtering

Use advanced time-based operations for fine-grained control:

```bash
# Combining time filters with other operations
rmxt list -t 1 | grep -E '\.(log|tmp|cache)$'    # Today's temp files

# Chain time-based operations
rmxt recover-all -t 3    # Recover last 3 days
rmxt tidy -t 7           # Then clean files older than 7 days
```

### Batch Time Operations

Process files in time-based batches:

```bash
#!/bin/bash
# time-based-management.sh

# Recover today's accidental deletions
echo "=== Files deleted today ==="
rmxt list -t 1

read -p "Recover all files from today? (y/N): " -n 1 -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rmxt recover-all -t 1
fi

# Progressive cleanup by age
echo "=== Cleaning old files ==="
rmxt tidy -t 90    # Remove files older than 90 days
rmxt tidy -t 60    # Remove files older than 60 days  
rmxt tidy -t 30    # Remove files older than 30 days

echo "=== Current trash status ==="
rmxt list | wc -l | awk '{print $1 " files remain in trash"}'
```

### Scheduling Automatic Cleanup

Set up automated trash management with cron:

```bash
# Edit crontab
crontab -e

# Add these lines for automatic management:

# Clean very old files daily at 2 AM
0 2 * * * /usr/local/bin/rmxt tidy -t 90

# Clean old files weekly on Sunday at 3 AM  
0 3 * * 0 /usr/local/bin/rmxt tidy -t 30

# Clean recent files monthly on 1st at 4 AM (more aggressive)
0 4 1 * * /usr/local/bin/rmxt tidy -t 7
```

## Scripting and Automation

### Error Handling in Scripts

Robust error handling for automated workflows:

```bash
#!/bin/bash
# robust-rmxt-script.sh

set -euo pipefail  # Exit on errors, undefined vars, pipe failures

# Function to safely remove with confirmation
safe_remove() {
    local files=("$@")
    local count=${#files[@]}
    
    if [ $count -eq 0 ]; then
        echo "No files to remove"
        return 0
    fi
    
    echo "About to remove $count files:"
    printf ' - %s\n' "${files[@]}"
    
    read -p "Continue? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rmxt "${files[@]}" || {
            echo "Error: Failed to remove files" >&2
            return 1
        }
        echo "Successfully removed $count files"
    else
        echo "Operation cancelled"
        return 1
    fi
}

# Example usage
temp_files=(*.tmp *.cache *.log)
if [ ${#temp_files[@]} -gt 0 ] && [ "${temp_files[0]}" != "*.tmp" ]; then
    safe_remove "${temp_files[@]}"
fi
```

### Integration with Build Systems

#### Makefile Integration

```makefile
# Makefile with rmxt integration

.PHONY: clean clean-all recover-build

# Standard clean (recoverable)
clean:
	rmxt -rf build/ dist/ target/
	rmxt *.log *.tmp
	@echo "Build artifacts moved to trash (recoverable with 'make recover-build')"

# Show what was cleaned
clean-status:
	@echo "Files in trash from today:"
	@rmxt list -t 1

# Recover build files if needed
recover-build:
	@echo "Recovering build files from today..."
	rmxt recover-all -t 1

# Permanent cleanup (use with caution)
clean-permanent:
	@read -p "Permanently delete build artifacts? This cannot be undone (y/N): " confirm && [ "$$confirm" = "y" ]
	rmxt -irf build/ dist/ target/
	rmxt -if *.log *.tmp
```

#### Cargo.toml Integration

```toml
# Add custom commands to Cargo.toml

[scripts]  # If using cargo-script
clean-safe = "rmxt -rf target/"
clean-temp = "rmxt *.tmp Cargo.lock"
recover-target = "rmxt recover target"
```

### Bulk Operations

Handle large numbers of files efficiently:

```bash
#!/bin/bash
# bulk-operations.sh

# Find and remove files by pattern efficiently
bulk_remove_pattern() {
    local pattern="$1"
    local max_age="${2:-30}"  # Default 30 days
    
    echo "Finding files matching: $pattern"
    
    # Use find for efficiency with large directory trees
    mapfile -t files < <(find . -name "$pattern" -type f -mtime +$max_age -print0 | xargs -0 -n1 echo)
    
    if [ ${#files[@]} -eq 0 ]; then
        echo "No files found matching pattern: $pattern"
        return 0
    fi
    
    echo "Found ${#files[@]} files matching pattern"
    echo "Files older than $max_age days will be moved to trash"
    
    # Process in chunks to avoid command line length limits
    local chunk_size=100
    for ((i=0; i<${#files[@]}; i+=chunk_size)); do
        chunk=("${files[@]:i:chunk_size}")
        echo "Processing chunk $((i/chunk_size + 1))..."
        rmxt "${chunk[@]}"
    done
    
    echo "Bulk removal completed"
}

# Usage examples
bulk_remove_pattern "*.log" 7      # Remove log files older than 7 days
bulk_remove_pattern "*.tmp" 1      # Remove temp files older than 1 day  
bulk_remove_pattern "*~" 0         # Remove backup files immediately
```

## Integration with Other Tools

### Find Command Integration

```bash
# Find large files and move to trash
find . -size +100M -exec rmxt {} \;

# Find files by age and remove
find . -name "*.log" -mtime +30 -exec rmxt {} \;

# Find empty directories and remove
find . -type d -empty -exec rmxt -d {} \;
```

### fd (Modern Find) Integration

```bash
# Install fd first: cargo install fd-find

# Remove files by extension
fd -e log -e tmp -x rmxt

# Remove files older than 7 days
fd --older 7d -x rmxt

# Remove large files
fd --size +50m -x rmxt

# Complex filtering
fd -e cache --older 30d --exclude node_modules -x rmxt
```

### ripgrep Integration

```bash
# Find files containing sensitive data and remove them
rg -l "password|secret|api_key" --type-not=md | xargs rmxt

# Find configuration files with old settings
rg -l "deprecated_option" -t config | xargs rmxt
```

### Git Integration

```bash
#!/bin/bash
# git-clean-rmxt.sh - Enhanced git clean using rmxt

# Remove untracked files (recoverable)
git ls-files --others --exclude-standard | xargs -r rmxt

# Remove ignored files (recoverable)  
git ls-files --ignored --exclude-standard | xargs -r rmxt

# Show what would be cleaned
git-clean-preview() {
    echo "Untracked files that would be removed:"
    git ls-files --others --exclude-standard
    echo
    echo "Ignored files that would be removed:"
    git ls-files --ignored --exclude-standard
}

# Safer alternative to git clean -fdx
git-clean-safe() {
    git-clean-preview
    read -p "Move these files to trash? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git ls-files --others --exclude-standard | xargs -r rmxt
        git ls-files --ignored --exclude-standard | xargs -r rmxt
        echo "Files moved to trash (recoverable)"
    fi
}
```

## Performance Optimization

### Large Directory Handling

Optimize performance when dealing with large directories:

```bash
#!/bin/bash
# performance-optimized.sh

# Count files before processing
count_files() {
    local dir="$1"
    find "$dir" -type f | wc -l
}

# Process large directories in batches
process_large_directory() {
    local target_dir="$1"
    local batch_size="${2:-1000}"
    
    echo "Analyzing directory: $target_dir"
    local total_files=$(count_files "$target_dir")
    echo "Total files: $total_files"
    
    if [ "$total_files" -gt "$batch_size" ]; then
        echo "Large directory detected. Processing in batches of $batch_size"
        find "$target_dir" -type f -print0 | \
        xargs -0 -n "$batch_size" rmxt
    else
        echo "Processing entire directory at once"
        rmxt -rf "$target_dir"
    fi
}

# Usage
process_large_directory "./huge_log_directory" 500
```

### Memory-Efficient Operations

Handle operations that might consume lots of memory:

```bash
# Stream processing for very large file lists
find /var/log -name "*.log" -mtime +30 -print0 | \
while IFS= read -r -d '' file; do
    rmxt "$file"
done

# Process trash listing efficiently
rmxt list | awk -v days=30 '
BEGIN { cutoff = systime() - (days * 24 * 60 * 60) }
{
    if (NF >= 3) {
        # Extract timestamp and compare
        # This is a simplified version - actual implementation would parse the date
        print $0
    }
}' | head -100
```

## Edge Cases and Limitations

### File Name Conflicts

Handle special characters and conflicts:

```bash
#!/bin/bash
# handle-special-names.sh

# Handle files with spaces and special characters
handle_special_files() {
    # Use arrays to properly handle filenames with spaces
    local files=()
    
    # Find files with spaces in names
    while IFS= read -r -d '' file; do
        files+=("$file")
    done < <(find . -name "* *" -type f -print0)
    
    if [ ${#files[@]} -gt 0 ]; then
        echo "Found ${#files[@]} files with spaces in names"
        rmxt "${files[@]}"
    fi
}

# Handle very long file names (system limits)
handle_long_names() {
    local max_length=255  # Most filesystems limit
    
    find . -type f | while read -r file; do
        if [ ${#file} -gt $max_length ]; then
            echo "Warning: Very long filename: $file"
            echo "Length: ${#file} characters"
            # Handle specially or warn user
        fi
    done
}

handle_special_files
handle_long_names
```

### Recovery Edge Cases

Handle complex recovery scenarios:

```bash
#!/bin/bash
# advanced-recovery.sh

# Recover files when original directory no longer exists
recover_with_fallback() {
    local filename="$1"
    local fallback_dir="${2:-$HOME/recovered_files}"
    
    echo "Attempting to recover: $filename"
    
    if ! rmxt recover "$filename" 2>/dev/null; then
        echo "Standard recovery failed. Trying fallback location..."
        mkdir -p "$fallback_dir"
        
        # Note: This is conceptual - actual implementation would need
        # to access trash metadata and handle the file restoration manually
        echo "File may need manual recovery to: $fallback_dir"
    fi
}

# Recover files in bulk with conflict handling
bulk_recover_safe() {
    local days="${1:-7}"
    local backup_suffix="_recovered_$(date +%Y%m%d_%H%M%S)"
    
    # Get list of files to recover
    local files=()
    while IFS= read -r file; do
        files+=("$file")
    done < <(rmxt list -t "$days" | awk 'NR>3 && NF>=3 {print $1}')
    
    for file in "${files[@]}"; do
        if [ -e "$file" ]; then
            # File exists at original location - handle conflict
            echo "Conflict: $file already exists"
            echo "Consider renaming or backing up existing file"
        else
            rmxt recover "$file"
        fi
    done
}
```

## Configuration and Customization

### Environment Variables

Customize `rmxt` behavior through environment variables:

```bash
# .bashrc or .zshrc additions

# Set default time for tidy operations
export RMXT_DEFAULT_TIDY_DAYS=60

# Set default behavior for confirmations
export RMXT_AUTO_CONFIRM=false

# Custom trash location (if supported in future versions)
export RMXT_TRASH_DIR="$HOME/.local/share/my_trash"
```

### Wrapper Functions

Create custom wrapper functions for enhanced functionality:

```bash
# Advanced wrapper functions for .bashrc/.zshrc

# Smart remove with automatic backup
smart_rm() {
    local backup_dir="$HOME/.rmxt_backups/$(date +%Y-%m-%d)"
    mkdir -p "$backup_dir"
    
    # Copy to backup before removing
    for file in "$@"; do
        if [ -e "$file" ]; then
            cp -r "$file" "$backup_dir/"
        fi
    done
    
    rmxt "$@"
    echo "Files backed up to: $backup_dir"
}

# Remove with automatic listing
rm_with_status() {
    rmxt "$@"
    echo "Current trash status:"
    rmxt list | tail -5
}

# Conditional remove based on file age
rm_if_old() {
    local days="${1:-30}"
    shift
    
    for file in "$@"; do
        if [ -e "$file" ] && find "$file" -mtime +$days -print0 | grep -qz .; then
            echo "Removing old file: $file"
            rmxt "$file"
        else
            echo "Keeping newer file: $file"
        fi
    done
}

# Progressive cleanup function
progressive_clean() {
    echo "=== Progressive Cleanup ==="
    echo "Step 1: Clean very old trash (90+ days)"
    rmxt tidy -t 90
    
    echo "Step 2: Clean old trash (30+ days)"  
    rmxt tidy -t 30
    
    echo "Step 3: Current trash status"
    rmxt list | wc -l | awk '{print "Files in trash:", $1}'
    
    read -p "Clean files older than 7 days? (y/N): " -n 1 -r
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo
        rmxt tidy -t 7
    fi
}
```

### Custom Aliases

Advanced aliases for power users:

```bash
# Comprehensive alias collection

# Basic shortcuts
alias rm='rmxt'
alias rml='rmxt list'
alias rmr='rmxt recover'
alias rma='rmxt recover-all'
alias rmt='rmxt tidy'

# Time-based operations
alias rm-today='rmxt list -t 1'
alias rm-week='rmxt list -t 7'  
alias recover-today='rmxt recover-all -t 1'
alias recover-week='rmxt recover-all -t 7'

# Cleanup operations
alias clean-temp='rmxt *.tmp *.cache *.log'
alias clean-old='rmxt tidy -t 30'
alias clean-aggressive='rmxt tidy -t 7'

# Safety operations  
alias rm-status='rmxt list | tail -10'
alias rm-count='rmxt list | wc -l'
alias rm-size='du -sh ~/.local/share/Trash/ 2>/dev/null || echo "Trash size not available"'

# Development specific
alias clean-build='rmxt -rf build/ dist/ target/ node_modules/'
alias clean-logs='find . -name "*.log" -exec rmxt {} \;'
alias clean-temp-recursive='find . -name "*.tmp" -o -name "*.cache" | xargs rmxt'
```

This completes the advanced features documentation, providing power users with comprehensive tools and techniques for maximizing `rmxt`'s capabilities.