# Installation Guide

This guide provides detailed installation instructions for `rmxt` across different platforms and scenarios.

## Quick Install

### Using Installation Script (Recommended)

The fastest way to get `rmxt` up and running is using our automated installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/install.sh | bash
```

Or download and inspect the script first:

```bash
# Download the script
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/install.sh -o install.sh

# Make it executable
chmod +x install.sh

# Run the installation
./install.sh
```

**What the script does:**
- ✅ Checks for Rust toolchain (installs if missing)
- ✅ Clones the repository to `~/rmxt`
- ✅ Builds the project in release mode
- ✅ Installs binary to `/usr/local/bin/rmxt`
- ✅ Verifies the installation works

**Script options:**
```bash
./install.sh --help              # Show all options
./install.sh --dry-run           # Preview what would be done
./install.sh --repo=<URL>        # Install from custom repository
```

**Environment variables:**
```bash
export RMXT_REPO_URL="https://github.com/yourfork/rmxt.git"
./install.sh                     # Uses custom repository
```

### From crates.io

The easiest way to install `rmxt` is using Cargo:

```bash
cargo install rmxt
```

This will:
- Download and compile the latest stable release
- Install the binary to `~/.cargo/bin/rmxt`
- Make `rmxt` available in your PATH (if Cargo's bin directory is in your PATH)

**Prerequisites:**
- Rust and Cargo installed on your system
- Internet connection for downloading dependencies

**Example output:**
```bash
$ cargo install rmxt
    Updating crates.io index
  Downloaded rmxt v0.1.7
  Downloaded 1 crate (25.3 KB) in 0.89s
  Installing rmxt v0.1.7
    Updating crates.io index
  Downloaded chrono v0.4.31
  Downloaded clap v4.4.7
  Downloaded trash v3.1.2
  # ... more dependencies
   Compiling rmxt v0.1.7
    Finished release [optimized] target(s) in 1m 23s
  Installing ~/.cargo/bin/rmxt
   Installed package `rmxt v0.1.7` (executable `rmxt`)
```

## Alternative Installation Methods

### From Source (Latest Development)

To get the latest development version with newest features:

```bash
# Clone the repository
git clone https://github.com/santoshxshrestha/rmxt
cd rmxt

# Build in release mode for optimal performance
cargo build --release

# Install to system (requires sudo on Unix-like systems)
sudo cp target/release/rmxt /usr/local/bin/

# Or install to user directory (no sudo required)
mkdir -p ~/.local/bin
cp target/release/rmxt ~/.local/bin/
```

**Note:** Make sure `~/.local/bin` is in your PATH if using the user directory option.

### Pre-compiled Binaries (Coming Soon)

Pre-compiled binaries for major platforms will be available in future releases.

## Platform-Specific Instructions

### Linux

**Ubuntu/Debian:**
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install rmxt
cargo install rmxt
```

**Arch Linux:**
```bash
# Install Rust via pacman
sudo pacman -S rust

# Install rmxt
cargo install rmxt
```

**Fedora/CentOS/RHEL:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install rmxt
cargo install rmxt
```

### macOS

**Using Homebrew (install Rust first):**
```bash
# Install Rust
brew install rust

# Install rmxt
cargo install rmxt
```

**Using rustup (recommended):**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install rmxt
cargo install rmxt
```

### Windows

**Using rustup:**
1. Download and run rustup-init.exe from [rustup.rs](https://rustup.rs/)
2. Follow the installation prompts
3. Open a new Command Prompt or PowerShell
4. Run: `cargo install rmxt`

**Using scoop (alternative):**
```powershell
# Install Rust
scoop install rust

# Install rmxt
cargo install rmxt
```

## Verification

After installation, verify `rmxt` is working correctly:

```bash
# Check version
rmxt --version

# Expected output: rmxt 0.1.7 (or current version)

# Check help
rmxt --help

# Test basic functionality (create a test file first)
echo "test content" > test_file.txt
rmxt test_file.txt
rmxt list
rmxt recover test_file.txt
```

## PATH Configuration

If `rmxt` is not found after installation, you may need to add Cargo's bin directory to your PATH:

### Unix-like systems (Linux, macOS)

Add to `~/.bashrc`, `~/.zshrc`, or your shell's configuration file:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload your shell:
```bash
source ~/.bashrc  # or ~/.zshrc
```

### Windows

Add `%USERPROFILE%\.cargo\bin` to your system PATH:

1. Open System Properties → Advanced → Environment Variables
2. Edit the PATH variable for your user
3. Add `%USERPROFILE%\.cargo\bin`
4. Restart your command prompt

## Troubleshooting

### Common Issues

**"rmxt: command not found"**
- Ensure Cargo's bin directory is in your PATH
- Restart your terminal after installation
- Verify installation with `ls ~/.cargo/bin/rmxt`

**Compilation errors**
- Update Rust: `rustup update`
- Clear Cargo cache: `cargo clean` (when building from source)
- Check system dependencies for your platform

**Permission denied**
- When installing to `/usr/local/bin/`, ensure you use `sudo`
- For user installation, use `~/.local/bin/` instead

**Slow compilation**
- This is normal for Rust projects, especially on first build
- Subsequent builds will be faster due to incremental compilation
- Consider using `cargo install --force rmxt` to reinstall

## Dependencies

`rmxt` automatically manages these runtime dependencies during installation:

- **chrono**: Date and time handling
- **clap**: Command-line argument parsing  
- **trash**: Cross-platform system trash integration
- **walkdir**: Recursive directory traversal
- **dirs**: Platform-specific directory utilities
- **colored**: Terminal text coloring
- **tabled**: Table formatting for output

No additional system dependencies are required on most platforms.

## Next Steps

After successful installation:

1. [Set up shell integration](shell-integration.md) to replace `rm` with `rmxt`
2. Read the [usage guide](usage.md) for comprehensive examples
3. Explore [advanced features](advanced-features.md) for power user functionality

## Uninstalling rmxt

### Using Uninstallation Script

If you installed `rmxt` using our installation script, you can easily remove it:

```bash
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/uninstall.sh | bash
```

Or download and run locally:

```bash
# Download the uninstall script
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/uninstall.sh -o uninstall.sh

# Make it executable
chmod +x uninstall.sh

# Run the uninstallation
./uninstall.sh
```

**Script options:**
```bash
./uninstall.sh --help           # Show help
./uninstall.sh --dry-run        # Preview what would be removed
```

**What the script removes:**
- 🗑️ Binary from `/usr/local/bin/rmxt`
- 🗑️ Repository directory at `~/rmxt`
- 🗑️ All associated files and build artifacts

### Manual Uninstallation

#### If installed via Cargo

```bash
cargo uninstall rmxt
```

#### If installed manually

```bash
# Remove the binary (adjust path as needed)
sudo rm -f /usr/local/bin/rmxt
# or
rm -f ~/.local/bin/rmxt

# Remove any downloaded source (if applicable)
rm -rf ~/rmxt
```

#### Remove Configuration (Optional)

If you want to completely remove all traces including trash data:

```bash
# Remove rmxt's trash directory (Linux/macOS)
rm -rf ~/.local/share/rmxt

# Remove any shell integration (if added)
# Edit your ~/.bashrc, ~/.zshrc, etc. to remove rmxt aliases
```

**Note:** Removing the trash directory will permanently delete all files currently in rmxt's trash. Make sure to recover any important files first using `rmxt list` and `rmxt recover`.

## Updating

To update to the latest version:

### If installed via script
```bash
# Re-run the installation script
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/install.sh | bash
```

### If installed via Cargo

```bash
cargo install rmxt --force
```

The `--force` flag ensures the existing installation is replaced with the new version.