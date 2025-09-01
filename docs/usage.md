# Usage Guide

This comprehensive guide covers all `rmxt` features with detailed examples and real-world scenarios.

## Table of Contents

1. [Basic File Operations](#basic-file-operations)
2. [Directory Operations](#directory-operations)
3. [Trash Management](#trash-management)
4. [File Recovery](#file-recovery)
5. [Time-Based Operations](#time-based-operations)
6. [Permanent Deletion](#permanent-deletion)
7. [Combining Options](#combining-options)
8. [Common Workflows](#common-workflows)
9. [Best Practices](#best-practices)

## Basic File Operations

### Removing Single Files

The most basic operation - moving files to trash instead of permanent deletion:

```bash
# Remove a single file
rmxt document.txt

# Remove multiple files
rmxt file1.txt file2.txt file3.txt

# Using wildcards (shell expansion)
rmxt *.log
rmxt temp_*.txt
rmxt data/*.csv
```

**Example session:**
```bash
$ ls
document.txt  image.png  notes.txt

$ rmxt document.txt
File 'document.txt' moved to trash

$ ls
image.png  notes.txt

$ rmxt list
┌─────────────┬──────────────────────────┬─────────────────────┐
│ name        │ original_location        │ deleted_at          │
├─────────────┼──────────────────────────┼─────────────────────┤
│ document.txt│ /home/user/work         │ 2024-01-15 14:30:22 │
└─────────────┴──────────────────────────┴─────────────────────┘
```

### File Types and Patterns

`rmxt` works with all file types and supports shell pattern matching:

```bash
# Remove specific file types
rmxt *.pdf *.doc *.docx          # Documents
rmxt *.jpg *.png *.gif           # Images  
rmxt *.mp4 *.avi *.mkv           # Videos
rmxt *.log *.tmp *.cache         # Temporary files

# Remove files with specific patterns
rmxt backup_*                    # All files starting with 'backup_'
rmxt *_old*                      # All files containing '_old'
rmxt test?.txt                   # test1.txt, test2.txt, etc.
```

## Directory Operations

### Empty Directories

Use the `-d` flag to remove empty directories:

```bash
# Remove single empty directory
rmxt -d empty_folder/

# Remove multiple empty directories
rmxt -d dir1/ dir2/ dir3/
```

**Example:**
```bash
$ mkdir empty_test_dir
$ rmxt -d empty_test_dir/
Directory 'empty_test_dir' moved to trash

$ rmxt list
┌──────────────┬────────────────────┬─────────────────────┐
│ name         │ original_location  │ deleted_at          │
├──────────────┼────────────────────┼─────────────────────┤
│ empty_test_dir│ /home/user/work   │ 2024-01-15 14:35:10 │
└──────────────┴────────────────────┴─────────────────────┘
```

### Recursive Directory Removal

Use the `-r` flag to remove directories and all their contents:

```bash
# Remove directory and all contents
rmxt -r project_folder/

# Remove multiple directories recursively  
rmxt -r dir1/ dir2/ dir3/

# Remove directories with patterns
rmxt -r temp_*/ backup_*/
```

**Example with nested structure:**
```bash
$ tree test_project/
test_project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── docs/
│   └── README.md
└── Cargo.toml

$ rmxt -r test_project/
Directory 'test_project' and its contents moved to trash

$ rmxt list
┌──────────────┬────────────────────┬─────────────────────┐
│ name         │ original_location  │ deleted_at          │
├──────────────┼────────────────────┼─────────────────────┤
│ test_project │ /home/user/work    │ 2024-01-15 14:40:15 │
└──────────────┴────────────────────┴─────────────────────┘
```

## Trash Management

### Listing Trash Contents

View all files currently in trash:

```bash
# List all files in trash
rmxt list

# List files with detailed output
rmxt list | head -20    # Show first 20 lines
```

**Example output:**
```bash
$ rmxt list
┌─────────────────┬────────────────────────────────┬─────────────────────┐
│ name            │ original_location              │ deleted_at          │
├─────────────────┼────────────────────────────────┼─────────────────────┤
│ document.pdf    │ /home/user/Documents           │ 2024-01-15 14:30:22 │
│ image.png       │ /home/user/Pictures            │ 2024-01-14 09:15:10 │
│ project_backup  │ /home/user/work                │ 2024-01-13 16:45:30 │
│ old_config.json │ /home/user/.config/app         │ 2024-01-12 11:20:05 │
│ temp_data.csv   │ /home/user/Downloads           │ 2024-01-11 08:30:45 │
└─────────────────┴────────────────────────────────┴─────────────────────┘
```

### Purging Files

Permanently delete specific files from trash:

```bash
# Permanently delete specific file from trash
rmxt purge document.pdf

# Permanently delete multiple files  
rmxt purge file1.txt file2.txt

# Note: This is permanent - files cannot be recovered after purging
```

**Example:**
```bash
$ rmxt list
┌──────────────┬────────────────────┬─────────────────────┐
│ name         │ original_location  │ deleted_at          │
├──────────────┼────────────────────┼─────────────────────┤
│ document.pdf │ /home/user/work    │ 2024-01-15 14:30:22 │
│ image.png    │ /home/user/work    │ 2024-01-14 09:15:10 │
└──────────────┴────────────────────┴─────────────────────┘

$ rmxt purge document.pdf
File 'document.pdf' permanently deleted from trash

$ rmxt list
┌───────────┬────────────────────┬─────────────────────┐
│ name      │ original_location  │ deleted_at          │
├───────────┼────────────────────┼─────────────────────┤
│ image.png │ /home/user/work    │ 2024-01-14 09:15:10 │
└───────────┴────────────────────┴─────────────────────┘
```

### Cleaning Old Files

Automatically remove old files from trash:

```bash
# Clean files older than 30 days (default)
rmxt tidy

# Clean files older than specific number of days
rmxt tidy -t 7      # Remove files older than 7 days
rmxt tidy -t 60     # Remove files older than 60 days
```

**Example:**
```bash
$ rmxt tidy -t 10
Cleaning trash: removing files older than 10 days
Removed 3 files from trash
Files removed:
- old_config.json (deleted 12 days ago)
- temp_data.csv (deleted 15 days ago)  
- backup_archive.zip (deleted 20 days ago)
```

## File Recovery

### Recovering Individual Files

Restore specific files from trash to their original locations:

```bash
# Recover single file
rmxt recover document.pdf

# Recover multiple files
rmxt recover file1.txt file2.txt image.png

# File names must match exactly as shown in 'rmxt list'
```

**Example:**
```bash
$ rmxt list
┌──────────────┬────────────────────────┬─────────────────────┐
│ name         │ original_location      │ deleted_at          │
├──────────────┼────────────────────────┼─────────────────────┤
│ document.pdf │ /home/user/Documents   │ 2024-01-15 14:30:22 │
│ image.png    │ /home/user/Pictures    │ 2024-01-14 09:15:10 │
└──────────────┴────────────────────────┴─────────────────────┘

$ rmxt recover document.pdf
File 'document.pdf' recovered to /home/user/Documents

$ ls /home/user/Documents
document.pdf  other_file.txt
```

### Mass Recovery

Restore all files or files within a time range:

```bash
# Recover all files from trash
rmxt recover-all

# Recover files from specific time period (last N days)
rmxt recover-all -t 7    # Recover files deleted in last 7 days
rmxt recover-all -t 30   # Recover files deleted in last 30 days
```

**Example:**
```bash
$ rmxt recover-all -t 5
Recovering files deleted within the last 5 days...
Recovered 3 files:
- document.pdf → /home/user/Documents
- script.py → /home/user/scripts  
- config.json → /home/user/.config/app

Files recovered successfully to their original locations
```

## Time-Based Operations

### Filtering by Time

Many commands support time-based filtering using the `-t` flag:

```bash
# List files deleted in last N days
rmxt list -t 7      # Files deleted in last 7 days
rmxt list -t 1      # Files deleted today
rmxt list -t 30     # Files deleted in last 30 days

# Clean files older than N days
rmxt tidy -t 14     # Remove files older than 14 days

# Recover files from last N days  
rmxt recover-all -t 3    # Recover files deleted in last 3 days
```

**Example workflow:**
```bash
# See what was deleted recently
$ rmxt list -t 2
┌──────────────┬────────────────────┬─────────────────────┐
│ name         │ original_location  │ deleted_at          │
├──────────────┼────────────────────┼─────────────────────┤
│ temp.txt     │ /home/user/work    │ 2024-01-15 16:20:10 │
│ draft.md     │ /home/user/work    │ 2024-01-15 14:15:30 │
└──────────────┴────────────────────┴─────────────────────┘

# Recover just the recent deletions
$ rmxt recover-all -t 2
Recovered 2 files from the last 2 days

# Clean older files but keep recent ones
$ rmxt tidy -t 30  # Keep files from last 30 days, remove older
```

## Permanent Deletion

### Bypassing Trash

Use the `-i` (ignore) flag to permanently delete files without using trash:

```bash
# Permanently delete single file (no recovery possible)
rmxt -i sensitive_data.txt

# Permanently delete multiple files
rmxt -i temp1.txt temp2.txt

# Permanently delete directory and contents  
rmxt -ir old_project/

# Permanently delete with force (no prompts)
rmxt -ifr dangerous_directory/
```

**⚠️ Warning Example:**
```bash
$ rmxt -i important_file.txt
WARNING: This will permanently delete 'important_file.txt' 
This action cannot be undone. Continue? (y/N): y
File 'important_file.txt' permanently deleted

$ rmxt list
# File will NOT appear in trash - it's gone forever
```

## Combining Options

### Common Flag Combinations

Combine flags for more complex operations:

```bash
# Recursive + force (remove directory without prompts)
rmxt -rf large_directory/

# Directory + force (remove empty directories without prompts)  
rmxt -df empty1/ empty2/ empty3/

# Ignore + recursive + force (permanently delete directory, no prompts)
rmxt -irf dangerous_data/    # ⚠️ VERY DANGEROUS

# Force removal of files (no prompts for confirmation)
rmxt -f protected_file.txt
```

**Example of progressive approach:**
```bash
# Start conservative
$ rmxt test_file.txt                    # Safe: goes to trash

# Add force if needed
$ rmxt -f several_files*.txt           # Skip prompts

# Add recursive for directories  
$ rmxt -rf temp_directory/             # Remove directory tree

# Only use -i flag when absolutely necessary
$ rmxt -irf truly_unwanted_data/       # ⚠️ Permanent deletion
```

## Common Workflows

### Development Cleanup

Clean up development artifacts:

```bash
# Remove build artifacts
rmxt -rf target/ build/ dist/ node_modules/

# Remove temporary files
rmxt *.tmp *.cache *.log
rmxt .DS_Store Thumbs.db

# Clean IDE files
rmxt .vscode/ .idea/ *.swp *~
```

### Project Management

Organize and clean projects:

```bash
# Archive old versions (move to trash first, can recover if needed)
rmxt -rf project_v1/ project_v2/ 

# Clean downloaded files
rmxt ~/Downloads/*.zip ~/Downloads/*.tar.gz

# Remove duplicates (move to trash for safety)
rmxt file_copy.txt file_backup.txt file_old.txt

# Later, if sure they're not needed:
rmxt tidy -t 7  # Permanently remove after 7 days
```

### Data Management

Handle data files safely:

```bash
# Move old data to trash (recoverable)
rmxt -r old_datasets/ legacy_exports/

# List what's in trash to verify
rmxt list

# If data is confirmed unwanted, clean periodically
rmxt tidy -t 30  # Keep 30 days of trash history

# For sensitive data requiring immediate deletion:
rmxt -irf sensitive_financial_data/  # ⚠️ Use with caution
```

## Best Practices

### Safety First

1. **Always use default behavior first**: Start with `rmxt file.txt` (goes to trash)
2. **Verify before permanent deletion**: Use `rmxt list` to see what's in trash
3. **Use time-based recovery**: `rmxt recover-all -t 7` for recent mistakes
4. **Clean trash periodically**: `rmxt tidy` to manage disk space

### Workflow Recommendations

```bash
# Daily workflow example:

# Morning: Clean up yesterday's work
rmxt *.tmp *.cache
rmxt -rf old_build/

# Check what was deleted
rmxt list -t 1

# End of day: Clean old trash  
rmxt tidy -t 30

# Weekly: Major cleanup
rmxt -rf old_projects/
rmxt recover important_file.txt  # If needed
rmxt tidy -t 7  # More aggressive cleanup
```

### Integration Tips

1. **Use aliases**: See [Shell Integration Guide](shell-integration.md)
2. **Combine with other tools**: 
   ```bash
   find . -name "*.tmp" -exec rmxt {} \;  # Find and remove temp files
   fd -e log -x rmxt                      # Use fd to find and remove logs
   ```
3. **Script integration**:
   ```bash
   #!/bin/bash
   # Cleanup script
   rmxt -rf build/ dist/
   rmxt *.log *.tmp
   rmxt list -t 1  # Show what was cleaned
   ```

### Recovery Strategies

1. **Immediate recovery**: `rmxt recover filename.txt`
2. **Bulk recovery**: `rmxt recover-all -t 1` (today's deletions)
3. **Selective recovery**: Use `rmxt list` then `rmxt recover` specific files
4. **Emergency recovery**: `rmxt recover-all` (everything in trash)

### Disk Space Management

```bash
# Check trash contents
rmxt list | wc -l  # Count files in trash

# Progressive cleanup
rmxt tidy -t 60    # Very old files
rmxt tidy -t 30    # Old files  
rmxt tidy -t 7     # Recent files (if space is critical)

# Manual cleanup of specific files
rmxt purge large_video.mp4 huge_dataset.csv
```

This completes the comprehensive usage guide. For more advanced features, see the [Advanced Features Guide](advanced-features.md).