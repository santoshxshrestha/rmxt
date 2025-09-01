# Usage

## Basic Commands

```bash
# Remove files/directories
rmxt file.txt           # Remove file to trash
rmxt -r directory/      # Remove directory to trash
rmxt -d empty_dir/      # Remove empty directory

# Trash management
rmxt list               # Show trash contents
rmxt recover file.txt   # Restore specific file
rmxt recover-all        # Restore all files
rmxt purge file.txt     # Permanently delete from trash
rmxt tidy               # Clean files older than 30 days
rmxt tidy -t 7          # Clean files older than 7 days

# Time-based operations
rmxt list -t 3          # Files deleted in last 3 days
rmxt recover-all -t 1   # Recover today's deletions

# Permanent deletion (no trash)
rmxt -i file.txt        # Skip trash, delete permanently
```

## Examples

### Clean Project Directory

```bash
rmxt -r node_modules/ target/ build/
rmxt *.log *.tmp
```

### Safe File Management  

```bash
rmxt old_files/         # Move to trash first
rmxt list               # Verify what was deleted
rmxt recover important.txt  # Oops, need it back
rmxt tidy -t 30         # Clean old trash after 30 days
```

### Bulk Operations

```bash
rmxt *.pdf *.doc        # Remove multiple file types
rmxt -r temp_* backup_* # Remove directories with patterns
```

## Flags

- `-r` - Recursive (directories)
- `-d` - Directory (empty only)
- `-f` - Force (no prompts)
- `-i` - Ignore trash (permanent delete)
- `-t N` - Time filter (N days)