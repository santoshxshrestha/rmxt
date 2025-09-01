#!/usr/bin/env bash
set -e

GREEN="\033[0;32m"
YELLOW="\033[1;33m"
CYAN="\033[0;36m"
RED="\033[0;31m"
BOLD="\033[1m"
RESET="\033[0m"
CHECK="${GREEN}✅${RESET}"
FAIL="${RED}❌${RESET}"
INFO="${CYAN}➜${RESET}"

DRY_RUN=false
REPO_URL="${RMXT_REPO_URL:-https://github.com/santoshxshrestha/rmxt.git}"

for arg in "$@"; do
    case $arg in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --repo=*)
            REPO_URL="${arg#*=}"
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --dry-run       Show what would be done without making changes"
            echo "  --repo=URL      Use custom repository URL (default: $REPO_URL)"
            echo "  -h, --help      Show this help message"
            exit 0
            ;;
        *)
            ;;
    esac
done

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}DRY RUN MODE - No changes will be made${RESET}"
    echo
fi

echo -e "${BOLD}${CYAN}"
echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "┃           rmxt Installer             ┃"
echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo -e "${RESET}"

echo -e "${INFO} Checking for Rust toolchain..."
if ! command -v cargo >/dev/null 2>&1; then
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}[DRY RUN] Would install Rust via rustup${RESET}"
    else
        echo -e "${YELLOW}Rust is not installed. Installing via rustup...${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        export PATH="$HOME/.cargo/bin:$PATH"
        echo -e "${CHECK} Rust installed!"
    fi
else
    echo -e "${CHECK} Rust is already installed."
fi

echo -e "${INFO} Cloning rmxt repository..."
if [ -d "$HOME/rmxt" ]; then
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}[DRY RUN] Would update existing repository at $HOME/rmxt${RESET}"
    else
        echo -e "${YELLOW}A previous rmxt directory exists. Updating repository...${RESET}"
        cd "$HOME/rmxt"
        git pull
    fi
else
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}[DRY RUN] Would clone $REPO_URL to $HOME/rmxt${RESET}"
    else
        git clone --depth 1 --branch main "$REPO_URL" "$HOME/rmxt"
    fi
fi

echo -e "${INFO} Building rmxt in release mode..."
if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}[DRY RUN] Would run 'cargo build --release' in $HOME/rmxt${RESET}"
else
    cd "$HOME/rmxt"
    cargo build --release
fi

BINARY_PATH="$HOME/rmxt/target/release/rmxt"
INSTALL_DIR="/usr/local/bin"
if [ "$DRY_RUN" = false ] && [ ! -f "$BINARY_PATH" ]; then
    echo -e "${FAIL} Error: Release binary not found at $BINARY_PATH."
    exit 1
fi

echo -e "${INFO} Installing rmxt to ${INSTALL_DIR} (may need sudo)..."
if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}[DRY RUN] Would copy $BINARY_PATH to $INSTALL_DIR/rmxt${RESET}"
    echo -e "${YELLOW}[DRY RUN] Would set executable permissions${RESET}"
else
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/rmxt"
    sudo chmod +x "$INSTALL_DIR/rmxt"
fi

echo -e "${CHECK} rmxt installed to ${INSTALL_DIR} and available in your PATH."

echo -e "${INFO} Verifying installation..."
if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}[DRY RUN] Would verify 'rmxt --version' command${RESET}"
else
    if command -v rmxt >/dev/null 2>&1; then
        VERSION=$(rmxt --version 2>/dev/null || echo "unknown version")
        echo -e "${CHECK} Installation verified! rmxt is working (${VERSION})"
    else
        echo -e "${FAIL} Warning: rmxt command not found in PATH. You may need to restart your terminal or add ${INSTALL_DIR} to your PATH."
        exit 1
    fi
fi

echo -e "${CHECK} You can now run '${BOLD}rmxt${RESET}' from anywhere in your terminal."
