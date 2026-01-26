#!/usr/bin/env bash
#
# savant-elite installer
# Downloads and installs savant (Kinesis Savant Elite foot pedal programmer) to your system
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Dicklesworthstone/savant-elite/master/install.sh | bash
#
# Options (via environment variables):
#   DEST=/path/to/dir      Install directory (default: ~/.local/bin)
#   SAVANT_SYSTEM=1        Install to /usr/local/bin (requires sudo)
#   SAVANT_VERSION=x.y.z   Install specific version (default: latest)
#

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# Configuration
REPO_OWNER="Dicklesworthstone"
REPO_NAME="savant-elite"
RELEASES_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases"
BINARY_NAME="savant"

log_info() { echo -e "${GREEN}[installer]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[installer]${NC} $1"; }
log_error() { echo -e "${RED}[installer]${NC} $1"; }
log_step() { echo -e "${BLUE}[installer]${NC} $1"; }

# Detect architecture
detect_arch() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64)
            echo "amd64"
            ;;
        arm64|aarch64)
            echo "arm64"
            ;;
        *)
            log_error "Unsupported architecture: $arch"
            log_error "Savant Elite only supports macOS on x86_64 (Intel) or arm64 (Apple Silicon)"
            exit 1
            ;;
    esac
}

# Detect OS
detect_os() {
    local os
    os=$(uname -s)
    case "$os" in
        Darwin)
            echo "darwin"
            ;;
        *)
            log_error "Unsupported OS: $os"
            log_error "Savant Elite is macOS-only (requires IOKit for USB access)"
            exit 1
            ;;
    esac
}

# Get latest version from GitHub releases (via redirect)
get_latest_version() {
    local redirect_url
    # Use the redirect from /releases/latest to get version without API
    if command -v curl &> /dev/null; then
        redirect_url=$(curl -fsSI "${RELEASES_URL}/latest" 2>/dev/null | grep -i '^location:' | tr -d '\r' | awk '{print $2}')
    elif command -v wget &> /dev/null; then
        redirect_url=$(wget --spider -S "${RELEASES_URL}/latest" 2>&1 | grep -i 'Location:' | tail -1 | awk '{print $2}')
    fi
    # Extract version from URL like .../releases/tag/v0.1.2
    if [[ -n "$redirect_url" ]]; then
        echo "$redirect_url" | sed -E 's|.*/tag/v?||' | tr -d '[:space:]'
    fi
}

# Download file
download_file() {
    local url="$1"
    local out="$2"

    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "$out" || return 1
    elif command -v wget &> /dev/null; then
        wget -qO "$out" "$url" || return 1
    else
        log_error "Neither curl nor wget found. Please install one of them."
        return 127
    fi
}

# Get install directory
get_install_dir() {
    if [[ -n "${DEST:-}" ]]; then
        echo "$DEST"
    elif [[ -n "${SAVANT_SYSTEM:-}" ]]; then
        echo "/usr/local/bin"
    else
        echo "${HOME}/.local/bin"
    fi
}

# Detect shell config file
get_shell_config() {
    local shell_name
    shell_name=$(basename "${SHELL:-/bin/bash}")

    case "$shell_name" in
        zsh)  echo "${HOME}/.zshrc" ;;
        bash)
            if [[ -f "${HOME}/.bashrc" ]]; then
                echo "${HOME}/.bashrc"
            else
                echo "${HOME}/.bash_profile"
            fi
            ;;
        fish) echo "${HOME}/.config/fish/config.fish" ;;
        *)    echo "${HOME}/.bashrc" ;;
    esac
}

# Add to PATH
add_to_path() {
    local install_dir="$1"
    local shell_config="$2"
    local shell_name
    shell_name=$(basename "${SHELL:-/bin/bash}")

    # Already in PATH?
    if [[ ":$PATH:" == *":${install_dir}:"* ]]; then
        return 0
    fi

    # Already in config?
    if [[ -f "$shell_config" ]] && grep -qF "$install_dir" "$shell_config" 2>/dev/null; then
        return 0
    fi

    # Ensure config dir exists
    mkdir -p "$(dirname "$shell_config")"

    # Add PATH line
    local path_line
    if [[ "$shell_name" == "fish" ]]; then
        path_line="fish_add_path ${install_dir}"
    else
        path_line="export PATH=\"${install_dir}:\$PATH\""
    fi

    {
        echo ""
        echo "# Added by savant-elite installer"
        echo "$path_line"
    } >> "$shell_config"

    log_info "Added ${install_dir} to PATH in ${shell_config}"
}

# Main
main() {
    echo ""
    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${CYAN}║${NC}                                                                ${BOLD}${CYAN}║${NC}"
    echo -e "${BOLD}${CYAN}║${NC}   ${BOLD}SAVANT ELITE${NC}  -  Kinesis Foot Pedal Programmer for macOS   ${BOLD}${CYAN}║${NC}"
    echo -e "${BOLD}${CYAN}║${NC}                                                                ${BOLD}${CYAN}║${NC}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "    ${RED}┌─────────┐${NC}      ${YELLOW}┌─────────┐${NC}      ${GREEN}┌─────────┐${NC}"
    echo -e "    ${RED}│  ${BOLD}LEFT${NC}${RED}   │${NC}      ${YELLOW}│ ${BOLD}MIDDLE${NC}${YELLOW}  │${NC}      ${GREEN}│  ${BOLD}RIGHT${NC}${GREEN}  │${NC}"
    echo -e "    ${RED}└─────────┘${NC}      ${YELLOW}└─────────┘${NC}      ${GREEN}└─────────┘${NC}"
    echo ""

    # Detect platform
    local os arch
    os=$(detect_os)
    arch=$(detect_arch)
    log_info "Detected platform: ${os}-${arch}"

    # Get version
    local version="${SAVANT_VERSION:-}"
    if [[ -z "$version" ]]; then
        log_step "Fetching latest release..."
        version=$(get_latest_version)
        if [[ -z "$version" ]]; then
            log_error "Could not determine latest version"
            exit 1
        fi
    fi
    log_info "Version: ${version}"

    # Setup paths
    local install_dir shell_config binary_path
    install_dir=$(get_install_dir)
    shell_config=$(get_shell_config)
    binary_path="${install_dir}/${BINARY_NAME}"

    log_step "Install directory: ${install_dir}"

    # Check for existing installation
    if [[ -x "$binary_path" ]]; then
        local existing_version
        existing_version=$("$binary_path" --version 2>/dev/null | awk '{print $2}' || echo "unknown")
        log_info "Existing installation: v${existing_version}"
        if [[ "$existing_version" == "$version" ]]; then
            log_info "Already at latest version, reinstalling..."
        else
            log_info "Upgrading: ${existing_version} → ${version}"
        fi
    fi

    # Create install directory
    if [[ ! -d "$install_dir" ]]; then
        log_step "Creating directory: ${install_dir}"
        mkdir -p "$install_dir"
    fi

    # Check write permission
    local use_sudo=""
    if [[ ! -w "$install_dir" ]]; then
        if command -v sudo &> /dev/null; then
            use_sudo="sudo"
            log_warn "Using sudo for installation to ${install_dir}"
        else
            log_error "Cannot write to ${install_dir} and sudo not available"
            exit 1
        fi
    fi

    # Build download URL
    local asset_name="savant-${os}-${arch}.tar.xz"
    local download_url="${RELEASES_URL}/download/v${version}/${asset_name}"
    local checksum_url="${RELEASES_URL}/download/v${version}/${asset_name}.sha256"

    # Download
    log_step "Downloading ${asset_name}..."
    local tmp_dir
    tmp_dir=$(mktemp -d)
    local tmp_tarball="${tmp_dir}/${asset_name}"

    if ! download_file "$download_url" "$tmp_tarball"; then
        log_error "Failed to download: ${download_url}"
        rm -rf "$tmp_dir"
        exit 1
    fi

    # Verify checksum
    log_step "Verifying checksum..."
    local tmp_checksum="${tmp_dir}/checksum.sha256"
    if download_file "$checksum_url" "$tmp_checksum" 2>/dev/null; then
        local expected actual
        expected=$(cat "$tmp_checksum" | awk '{print $1}')
        actual=$(shasum -a 256 "$tmp_tarball" | awk '{print $1}')
        if [[ "$expected" == "$actual" ]]; then
            log_info "Checksum verified: ${actual:0:16}..."
        else
            log_error "Checksum mismatch!"
            log_error "Expected: $expected"
            log_error "Got:      $actual"
            rm -rf "$tmp_dir"
            exit 1
        fi
    else
        log_warn "Checksum file not found, skipping verification"
    fi

    # Extract
    log_step "Extracting..."
    tar -xJf "$tmp_tarball" -C "$tmp_dir"

    # Install
    log_step "Installing to ${binary_path}..."
    $use_sudo mv "${tmp_dir}/${BINARY_NAME}" "$binary_path"
    $use_sudo chmod +x "$binary_path"

    # Cleanup
    rm -rf "$tmp_dir"

    # Add to PATH
    add_to_path "$install_dir" "$shell_config"

    # Verify
    if [[ -x "$binary_path" ]]; then
        echo ""
        echo -e "${GREEN}${BOLD}Installation complete!${NC}"
        echo ""
        echo -e "${BOLD}Quick Start:${NC}"
        echo ""
        echo -e "  ${DIM}# Check device status${NC}"
        echo -e "  ${CYAN}savant info${NC}"
        echo ""
        echo -e "  ${DIM}# Program pedals (device must be in programming mode)${NC}"
        echo -e "  ${CYAN}savant program --left cmd+c --middle cmd+a --right cmd+v${NC}"
        echo ""
        echo -e "  ${DIM}# Monitor pedal input in real-time${NC}"
        echo -e "  ${CYAN}savant monitor${NC}"
        echo ""
        echo -e "${BOLD}Programming Mode:${NC}"
        echo -e "  1. Flip the pedal over and find the recessed switch near the Kinesis sticker"
        echo -e "  2. Use a paperclip to flip the switch from ${GREEN}Play${NC} → ${RED}Program${NC}"
        echo -e "  3. Unplug and replug USB, then run ${CYAN}savant status${NC} to verify"
        echo ""

        if [[ ":$PATH:" != *":${install_dir}:"* ]]; then
            echo -e "${YELLOW}Note:${NC} Restart your terminal or run:"
            echo -e "  ${CYAN}source ${shell_config}${NC}"
            echo ""
        fi
    else
        log_error "Installation failed"
        exit 1
    fi
}

main "$@"
