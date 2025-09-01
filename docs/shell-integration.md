# Shell Integration Guide

This guide covers comprehensive shell integration for `rmxt`, making it a seamless replacement for the traditional `rm` command across different shells and platforms.

## Table of Contents

1. [Basic Alias Setup](#basic-alias-setup)
2. [Advanced Shell Integration](#advanced-shell-integration)
3. [Shell-Specific Configurations](#shell-specific-configurations)
4. [Safety Wrappers](#safety-wrappers)
5. [Tab Completion](#tab-completion)
6. [Cross-Shell Compatibility](#cross-shell-compatibility)
7. [Migration Strategies](#migration-strategies)

## Basic Alias Setup

### Simple Replacement

The most straightforward approach - replace `rm` with `rmxt`:

```bash
# Add to your shell configuration file
alias rm='rmxt'
```

### Graduated Approach

For users who want to transition gradually:

```bash
# Keep both commands available
alias rm='rmxt'           # New default behavior
alias rm-old='/bin/rm'    # Original rm for when needed
alias rm-permanent='rmxt -i'  # Permanent deletion shortcut
```

### Safety-First Approach

Add confirmation prompts and safety checks:

```bash
# Interactive removal with confirmation
alias rm='rmxt -f'        # Force mode (skip system prompts)
alias rmi='rmxt -i'       # Interactive permanent deletion
alias rmf='rmxt -rf'      # Force recursive removal
```

## Advanced Shell Integration

### Intelligent Wrapper Function

Create a wrapper that provides enhanced functionality:

```bash
# Advanced rm replacement function
rm() {
    local force=false
    local recursive=false
    local permanent=false
    local show_help=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help=true
                shift
                ;;
            -f|--force)
                force=true
                shift
                ;;
            -r|--recursive)
                recursive=true
                shift
                ;;
            -i|--permanent)
                permanent=true
                shift
                ;;
            -rf|-fr)
                force=true
                recursive=true
                shift
                ;;
            *)
                break
                ;;
        esac
    done
    
    # Show help if requested
    if [ "$show_help" = true ]; then
        rmxt --help
        return 0
    fi
    
    # Build rmxt command
    local cmd="rmxt"
    [ "$force" = true ] && cmd="$cmd -f"
    [ "$recursive" = true ] && cmd="$cmd -r"  
    [ "$permanent" = true ] && cmd="$cmd -i"
    
    # Execute with remaining arguments
    $cmd "$@"
    
    # Show status after removal
    if [ $? -eq 0 ] && [ "$permanent" = false ]; then
        echo "Files moved to trash. Use 'rm --recover' to restore or 'rm --list' to view trash."
    fi
}

# Add utility functions
alias rm-list='rmxt list'
alias rm-recover='rmxt recover'
alias rm-recover-all='rmxt recover-all'
alias rm-tidy='rmxt tidy'
alias rm-purge='rmxt purge'
```

### Context-Aware Removal

Automatically adjust behavior based on file types or locations:

```bash
smart_rm() {
    local files=("$@")
    local safe_remove=true
    local important_patterns=("*.config" "*.key" "*.pem" "*.crt" "*.env")
    
    # Check for important files
    for file in "${files[@]}"; do
        for pattern in "${important_patterns[@]}"; do
            if [[ "$file" == $pattern ]]; then
                echo "⚠️  Important file detected: $file"
                read -p "Are you sure you want to remove this? (y/N): " -n 1 -r
                echo
                if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                    echo "Skipping $file"
                    continue 2
                fi
            fi
        done
    done
    
    # Check file sizes
    for file in "${files[@]}"; do
        if [ -f "$file" ]; then
            local size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null)
            if [ "$size" -gt 100000000 ]; then  # 100MB
                echo "📦 Large file detected: $file ($(numfmt --to=iec $size))"
                read -p "Move large file to trash? (Y/n): " -n 1 -r
                echo
                if [[ $REPLY =~ ^[Nn]$ ]]; then
                    echo "Skipping $file"
                    continue
                fi
            fi
        fi
    done
    
    rmxt "$@"
}

alias rm='smart_rm'
```

## Shell-Specific Configurations

### Bash Configuration

Add to `~/.bashrc`:

```bash
# rmxt integration for Bash
if command -v rmxt &> /dev/null; then
    # Basic alias
    alias rm='rmxt'
    
    # Enhanced aliases  
    alias rml='rmxt list'
    alias rmr='rmxt recover'
    alias rmt='rmxt tidy'
    alias rms='rmxt list | wc -l && echo " files in trash"'
    
    # Bash-specific function with completion
    rm_with_completion() {
        rmxt "$@"
        if [ $? -eq 0 ]; then
            local count=$(rmxt list | wc -l)
            echo "✓ Files moved to trash (total: $count files)"
        fi
    }
    
    # Override alias with function for advanced features
    alias rm='rm_with_completion'
    
    # Bash completion for rmxt subcommands
    _rmxt_completion() {
        local cur prev opts
        COMPREPLY=()
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
        opts="list recover recover-all purge tidy"
        
        if [ $COMP_CWORD -eq 1 ]; then
            COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
        fi
        
        return 0
    }
    complete -F _rmxt_completion rmxt
fi
```

### Zsh Configuration

Add to `~/.zshrc`:

```zsh
# rmxt integration for Zsh
if command -v rmxt &> /dev/null; then
    # Basic integration
    alias rm='rmxt'
    
    # Zsh-specific enhancements
    autoload -U colors && colors
    
    # Colored output function
    rm_colored() {
        rmxt "$@"
        if [ $? -eq 0 ]; then
            echo "${fg[green]}✓ Files moved to trash${reset_color}"
            echo "${fg[blue]}Use 'rmxt list' to view trash contents${reset_color}"
        else
            echo "${fg[red]}✗ Error moving files to trash${reset_color}"
        fi
    }
    
    alias rm='rm_colored'
    
    # Zsh completion
    if [ -d ~/.zsh/completions ]; then
        # Create completion file
        cat > ~/.zsh/completions/_rmxt << 'EOF'
#compdef rmxt
    
_rmxt() {
    local context state line
    
    _arguments \
        '(-h --help)'{-h,--help}'[Show help]' \
        '(-V --version)'{-V,--version}'[Show version]' \
        '(-i --ignore)'{-i,--ignore}'[Permanently delete]' \
        '(-r --recursive)'{-r,--recursive}'[Remove recursively]' \
        '(-f --force)'{-f,--force}'[Force removal]' \
        '(-d --dir)'{-d,--dir}'[Remove empty directories]' \
        '*:file:_files' \
        '::command:(list recover recover-all purge tidy)'
}
    
_rmxt "$@"
EOF
        
        # Add completion directory to fpath
        fpath=(~/.zsh/completions $fpath)
        autoload -U compinit && compinit
    fi
fi
```

### Fish Shell Configuration

Add to `~/.config/fish/config.fish`:

```fish
# rmxt integration for Fish shell
if command -v rmxt &> /dev/null
    # Basic alias
    alias rm='rmxt'
    
    # Fish-specific function with better error handling
    function rm_fish
        set -l result (rmxt $argv)
        set -l status_code $status
        
        if test $status_code -eq 0
            echo "✓ Files moved to trash"
            set -l count (rmxt list | wc -l)
            echo "📁 Total files in trash: $count"
        else
            echo "✗ Error: $result" >&2
            return $status_code
        end
    end
    
    alias rm='rm_fish'
    
    # Fish completions
    complete -c rmxt -s h -l help -d 'Show help'
    complete -c rmxt -s V -l version -d 'Show version'
    complete -c rmxt -s i -l ignore -d 'Permanently delete'
    complete -c rmxt -s r -l recursive -d 'Remove recursively'
    complete -c rmxt -s f -l force -d 'Force removal'
    complete -c rmxt -s d -l dir -d 'Remove empty directories'
    
    # Subcommand completions
    complete -c rmxt -n '__fish_use_subcommand' -a 'list' -d 'List trash contents'
    complete -c rmxt -n '__fish_use_subcommand' -a 'recover' -d 'Recover file from trash'
    complete -c rmxt -n '__fish_use_subcommand' -a 'recover-all' -d 'Recover all files'
    complete -c rmxt -n '__fish_use_subcommand' -a 'purge' -d 'Permanently delete from trash'
    complete -c rmxt -n '__fish_use_subcommand' -a 'tidy' -d 'Clean old files from trash'
end
```

### PowerShell Configuration (Windows)

Add to your PowerShell profile:

```powershell
# rmxt integration for PowerShell
if (Get-Command rmxt -ErrorAction SilentlyContinue) {
    # Basic alias
    Set-Alias rm rmxt
    
    # Enhanced function with Windows-specific features
    function Remove-ItemSafe {
        param(
            [Parameter(ValueFromPipeline=$true, ValueFromPipelineByPropertyName=$true)]
            [string[]]$Path,
            [switch]$Recurse,
            [switch]$Force,
            [switch]$Permanent
        )
        
        begin {
            $rmxtArgs = @()
            if ($Recurse) { $rmxtArgs += '-r' }
            if ($Force) { $rmxtArgs += '-f' }
            if ($Permanent) { $rmxtArgs += '-i' }
        }
        
        process {
            foreach ($item in $Path) {
                $fullArgs = $rmxtArgs + @($item)
                & rmxt @fullArgs
                
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "✓ $item moved to trash" -ForegroundColor Green
                } else {
                    Write-Host "✗ Failed to move $item" -ForegroundColor Red
                }
            }
        }
    }
    
    Set-Alias rm Remove-ItemSafe
    Set-Alias del Remove-ItemSafe
    
    # PowerShell-specific aliases
    Set-Alias rmxt-list 'rmxt list'
    Set-Alias rmxt-recover 'rmxt recover'
    Set-Alias rmxt-tidy 'rmxt tidy'
}
```

## Safety Wrappers

### Confirmation Wrapper

Add confirmation prompts for dangerous operations:

```bash
safe_rm() {
    local permanent=false
    local dangerous_patterns=("/" "/home" "/usr" "/etc" "/var" "/opt")
    local confirm_patterns=("*.config" "*.env" "*.key" "*.pem")
    
    # Check for dangerous paths
    for arg in "$@"; do
        for pattern in "${dangerous_patterns[@]}"; do
            if [[ "$arg" == "$pattern" ]] || [[ "$arg" == "$pattern"/* ]]; then
                echo "🚫 DANGER: Attempting to remove system directory: $arg"
                echo "This operation is not allowed for safety."
                return 1
            fi
        done
    done
    
    # Check for permanent deletion flag
    if [[ "$*" == *"-i"* ]]; then
        permanent=true
        echo "⚠️  PERMANENT DELETION MODE"
        echo "Files will be permanently deleted (not recoverable)"
        read -p "Are you absolutely sure? Type 'DELETE' to confirm: " confirm
        if [[ "$confirm" != "DELETE" ]]; then
            echo "Operation cancelled"
            return 1
        fi
    fi
    
    # Check for important files
    for arg in "$@"; do
        if [ -f "$arg" ]; then
            for pattern in "${confirm_patterns[@]}"; do
                if [[ "$arg" == $pattern ]]; then
                    echo "🔒 Important file detected: $arg"
                    read -p "Confirm removal (y/N): " -n 1 -r
                    echo
                    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                        echo "Skipping $arg"
                        return 1
                    fi
                fi
            done
        fi
    done
    
    rmxt "$@"
}

alias rm='safe_rm'
```

### Backup Wrapper

Automatically backup important files before removal:

```bash
backup_rm() {
    local backup_dir="$HOME/.rmxt_backups/$(date +%Y-%m-%d)"
    local backup_patterns=("*.config" "*.env" "*.key" "*.json" "*.yaml" "*.yml")
    local backup_needed=false
    
    # Check if any files need backup
    for arg in "$@"; do
        if [ -f "$arg" ]; then
            for pattern in "${backup_patterns[@]}"; do
                if [[ "$arg" == $pattern ]]; then
                    backup_needed=true
                    break 2
                fi
            done
        fi
    done
    
    # Create backup if needed
    if [ "$backup_needed" = true ]; then
        mkdir -p "$backup_dir"
        echo "📦 Creating backup of important files..."
        
        for arg in "$@"; do
            if [ -f "$arg" ]; then
                for pattern in "${backup_patterns[@]}"; do
                    if [[ "$arg" == $pattern ]]; then
                        cp "$arg" "$backup_dir/"
                        echo "   Backed up: $arg"
                        break
                    fi
                done
            fi
        done
        
        echo "   Backup location: $backup_dir"
    fi
    
    rmxt "$@"
}

alias rm='backup_rm'
```

## Tab Completion

### Advanced Bash Completion

```bash
# Enhanced bash completion for rmxt
_rmxt_advanced() {
    local cur prev words cword
    _init_completion || return
    
    case $prev in
        list|tidy|recover-all)
            COMPREPLY=( $(compgen -W "-t --time" -- "$cur") )
            return 0
            ;;
        recover|purge)
            # Complete with trash file names
            local trash_files
            if command -v rmxt &> /dev/null; then
                trash_files=$(rmxt list 2>/dev/null | awk 'NR>3 && NF>=3 {print $1}' | tr '\n' ' ')
                COMPREPLY=( $(compgen -W "$trash_files" -- "$cur") )
            fi
            return 0
            ;;
        -t|--time)
            COMPREPLY=( $(compgen -W "1 7 30 60 90" -- "$cur") )
            return 0
            ;;
    esac
    
    if [[ $cur == -* ]]; then
        COMPREPLY=( $(compgen -W "-i --ignore -r --recursive -f --force -d --dir -h --help -V --version" -- "$cur") )
    else
        # File completion for regular files, directory completion for commands that need it
        case ${words[1]} in
            list|tidy|recover-all)
                return 0  # No file completion needed
                ;;
            recover|purge)
                # Already handled above
                return 0
                ;;
            *)
                _filedir
                ;;
        esac
    fi
}

complete -F _rmxt_advanced rmxt
complete -F _rmxt_advanced rm  # If using rm alias
```

### Zsh Completion with Context

```zsh
# Advanced Zsh completion
_rmxt_context() {
    local context state line
    typeset -A opt_args
    
    _arguments \
        '(-h --help)'{-h,--help}'[Show help information]' \
        '(-V --version)'{-V,--version}'[Show version information]' \
        '(-i --ignore)'{-i,--ignore}'[Permanently delete without using trash]' \
        '(-r --recursive)'{-r,--recursive}'[Remove directories and contents recursively]' \
        '(-f --force)'{-f,--force}'[Force removal without prompts]' \
        '(-d --dir)'{-d,--dir}'[Remove empty directories]' \
        '*:files:_files' \
        '1: :_rmxt_commands' && return 0
}

_rmxt_commands() {
    local commands
    commands=(
        'list:List files in trash with deletion timestamps'
        'recover:Restore specific file from trash'
        'recover-all:Restore all files from trash'
        'purge:Permanently delete specific file from trash'
        'tidy:Clean up trash by removing old files'
    )
    _describe 'rmxt commands' commands
}

compdef _rmxt_context rmxt
```

## Cross-Shell Compatibility

### Universal Configuration Script

Create a script that works across multiple shells:

```bash
#!/bin/bash
# universal-rmxt-setup.sh

setup_rmxt_integration() {
    local shell_name="$1"
    local config_file="$2"
    
    echo "Setting up rmxt integration for $shell_name..."
    
    # Common aliases
    cat >> "$config_file" << 'EOF'

# rmxt integration
if command -v rmxt &> /dev/null; then
    alias rm='rmxt'
    alias rml='rmxt list'
    alias rmr='rmxt recover' 
    alias rma='rmxt recover-all'
    alias rmt='rmxt tidy'
    alias rmp='rmxt purge'
    
    # Universal function (works in bash, zsh, and others)
    rm_status() {
        echo "Current trash status:"
        rmxt list | tail -5
        echo "---"
        echo "Total files in trash: $(rmxt list | wc -l)"
    }
    
    alias rms='rm_status'
fi

EOF
    
    echo "✓ Integration added to $config_file"
    echo "  Please run: source $config_file"
}

# Detect shell and apply configuration
case "$SHELL" in
    */bash)
        setup_rmxt_integration "Bash" "$HOME/.bashrc"
        ;;
    */zsh)
        setup_rmxt_integration "Zsh" "$HOME/.zshrc"
        ;;
    */fish)
        echo "Fish shell detected. Please manually add configuration to ~/.config/fish/config.fish"
        ;;
    *)
        echo "Unknown shell: $SHELL"
        echo "Please manually add rmxt aliases to your shell configuration file"
        ;;
esac
```

### Environment Detection

Create smart configuration that adapts to the environment:

```bash
# Smart environment detection for rmxt
smart_rmxt_setup() {
    # Detect operating system
    case "$(uname -s)" in
        Linux*)
            export RMXT_PLATFORM="linux"
            export RMXT_TRASH_LOCATION="$HOME/.local/share/Trash"
            ;;
        Darwin*)
            export RMXT_PLATFORM="macos" 
            export RMXT_TRASH_LOCATION="$HOME/.Trash"
            ;;
        MINGW*|CYGWIN*)
            export RMXT_PLATFORM="windows"
            export RMXT_TRASH_LOCATION="Recycle Bin"
            ;;
    esac
    
    # Detect terminal capabilities
    if [[ -t 1 ]] && command -v tput &> /dev/null && tput colors &> /dev/null; then
        export RMXT_COLOR_SUPPORT=true
        export RMXT_RED=$(tput setaf 1)
        export RMXT_GREEN=$(tput setaf 2)
        export RMXT_YELLOW=$(tput setaf 3)
        export RMXT_BLUE=$(tput setaf 4)
        export RMXT_RESET=$(tput sgr0)
    else
        export RMXT_COLOR_SUPPORT=false
    fi
    
    # Platform-specific aliases
    case "$RMXT_PLATFORM" in
        linux)
            alias open-trash='xdg-open $RMXT_TRASH_LOCATION'
            ;;
        macos)
            alias open-trash='open $RMXT_TRASH_LOCATION'
            ;;
        windows)
            alias open-trash='explorer shell:RecycleBinFolder'
            ;;
    esac
}

smart_rmxt_setup
```

## Migration Strategies

### Gradual Migration

For users transitioning from traditional `rm`:

```bash
# Phase 1: Parallel installation (both commands available)
alias rmxt-safe='rmxt'     # New safe command
alias rm-old='/bin/rm'     # Keep original for emergencies

# Phase 2: Gradual replacement with warnings
rm() {
    echo "⚠️  Using new safe removal (rmxt). Files go to trash."
    echo "   Use 'rm-old' for original rm behavior if needed."
    rmxt "$@"
}

# Phase 3: Full replacement (add to final configuration)
alias rm='rmxt'
```

### Team Migration

For teams adopting rmxt:

```bash
#!/bin/bash
# team-migration.sh

# Check if all team members have rmxt installed
check_team_readiness() {
    local team_members=("user1@company.com" "user2@company.com")
    local config_repo="git@company.com:team/dotfiles.git"
    
    echo "Checking team readiness for rmxt migration..."
    
    # This would integrate with your team's infrastructure
    # to verify rmxt installation across development machines
}

# Create team-wide configuration
create_team_config() {
    cat > .rmxt-team-config << 'EOF'
# Team-wide rmxt configuration
# Add this to your shell configuration file

# Standard team aliases
alias rm='rmxt'
alias cleanup='rmxt -rf build/ dist/ node_modules/ .next/'
alias recover-today='rmxt recover-all -t 1'

# Team-specific safety settings
export RMXT_TEAM_MODE=true

# Override for CI/CD environments
if [ "$CI" = "true" ]; then
    alias rm='/bin/rm'  # Use original rm in CI
fi
EOF
    
    echo "Team configuration created in .rmxt-team-config"
    echo "Team members should source this file in their shell configuration"
}

create_team_config
```

### Rollback Strategy

Provide easy rollback options:

```bash
# rmxt-rollback.sh
rollback_rmxt() {
    echo "Rolling back rmxt integration..."
    
    # Remove aliases from shell configuration files
    local config_files=("$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.config/fish/config.fish")
    
    for config in "${config_files[@]}"; do
        if [ -f "$config" ]; then
            # Create backup
            cp "$config" "${config}.rmxt-backup"
            
            # Remove rmxt lines
            sed -i '/# rmxt integration/,/# end rmxt integration/d' "$config"
            sed -i '/alias rm=.*rmxt/d' "$config"
            
            echo "✓ Cleaned $config"
        fi
    done
    
    echo "Rollback complete. Please restart your shell or run:"
    echo "  source ~/.bashrc  # or your shell's config file"
    echo ""
    echo "Original configuration backed up with .rmxt-backup extension"
}

# Uncomment to run rollback
# rollback_rmxt
```

This completes the comprehensive shell integration guide, providing users with everything needed to seamlessly integrate `rmxt` into their daily workflow across different shells and environments.