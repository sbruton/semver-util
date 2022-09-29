@_default:
    JUST_CHOOSER="sk" just --choose

# minimum rustc version supported
_rust_min := "1.60.0"

@_check_brew:
    type brew 2>&1 > /dev/null || just _install_brew

@_check_toolchain: _check_toolchain_rust _check_toolchain_gh

@_check_toolchain_gh:
    type gh 2>&1 > /dev/null || just _install_gh

@_check_toolchain_rust:
    type rustc 2>&1 > /dev/null || just _install_rust
    semver compare `rustc --version | awk '{print $2}'` lt 1.64.0 2>&1 > /dev/null && just _update_rust || $()

@_install_gh:
    if [[ "{{os()}}" == "macos" ]]; then \
        just _install_gh_macos; \
    elif [[ "{{os()}}" == "linux" ]]; then \
        just _install_gh_linux; \
    else \
        echo "\n\033[41m\033[1;37mERR: GitHub CLI cannot be installed automatically on this operating system.\033[0m\n"; \
    fi

_install_brew:
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

_install_gh_macos: _check_brew
    brew install gh

@_install_rust:
    if [[ "{{os()}}" == "macos" ]]; then \
        just _install_rust_macos; \
    elif [[ "{{os()}}" == "linux" ]]; then \
        just _install_rust_linux; \
    else \
        echo "\n\033[41m\033[1;37mERR: Rust cannot be installed automatically on this operating system.\033[0m\n"; \
    fi

_install_rust_macos:
    brew install rustup-init
    rustup-init -y

_install_rust_linux:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -c '-y'

_update_rust:
    rustup update
    @for rust_version in $(semver seq --minor --minor-max 100 {{_rust_min}} `rustc --version | awk '{print $2}'`); do \
        echo "\n\033[44m\033[1;37mUpdating rustc v${rust_version}\033[0m\n"; \
        cargo +$rust_version version 2>&1 > /dev/null || rustup toolchain install $rust_version; \
    done

# Build for local target
build *FLAGS: _check_toolchain 
    cargo build --bin semver {{FLAGS}}

# Build for all supported targets
build-all: build-apple-arm build-apple-x86 build-linux-x86-gnu build-linux-arm-gnu build-windows-x86-gnu

# Build for Apple macOS targeting the 64-bit Apple ISA (e.g., Apple Silicon Macs)
build-apple-arm *FLAGS: _check_toolchain 
    cargo build --bin semver --release --target aarch64-apple-darwin {{FLAGS}}

# Build for Apple macOS targeting the 64-bit x86 (amd64) ISA (e.g., Legacy Intel Macs)
build-apple-x86 *FLAGS: _check_toolchain 
    cargo build --bin semver --release --target x86_64-apple-darwin {{FLAGS}}

# Build for GNU/Linux targeting the 64-bit ARMv8 (AArch64) ISA (e.g., AWS Graviton)
build-linux-arm-gnu *FLAGS: _check_toolchain 
    cargo build --bin semver --release --target aarch64-unknown-linux-gnu {{FLAGS}}

# Build for GNU/Linux targeting the 32-bit ARMv7 ISA (e.g., Raspberry Pi)
build-linux-armv7-gnu *FLAGS: _check_toolchain 
    cargo build --bin semver --release --target armv7-unknown-linux-gnueabihf {{FLAGS}}

# Build for GNU/Linux targeting the 64-bit x86 (amd64) ISA (e.g., Intel/AMD PCs)
build-linux-x86-gnu *FLAGS: _check_toolchain 
    cargo build --bin semver --release --target x86_64-unknown-linux-gnu {{FLAGS}}

# Build for GNU/Windows targeting the 64-bit x86 (amd64) ISA (e.g, Intel/AMD PCs)
build-windows-x86-gnu *FLAGS: _check_toolchain 
    cargo build --bin semver --release --target x86_64-pc-windows-gnu {{FLAGS}}

# Run all lints and tests
check: _check_toolchain
    @for rust_version in $(semver seq --minor --minor-max 100 {{_rust_min}} `rustc --version | awk '{print $2}'`); do \
        echo "\n\033[44m\033[1;37mChecking rustc v${rust_version}\033[0m\n"; \
        cargo +$rust_version check; \
    done

# Remove all build artifacts
clean: _check_toolchain 
    cargo clean

# Publish build artifacts to a GitHub release tag
publish-artifacts tag=`git tag --list | tail -n 1`: build-all
    if [[ -d dist ]]; then rm -rf dist; fi
    mkdir dist
    cp target/aarch64-apple-darwin/release/semver dist/semver.{{tag}}.aarch64-apple-darwin
    cp target/aarch64-unknown-linux-gnu/release/semver dist/semver.{{tag}}.aarch64-unknown-linux-gnu
    cp target/x86_64-apple-darwin/release/semver dist/semver.{{tag}}.x86_64-apple-darwin
    cp target/x86_64-pc-windows-gnu/release/semver.exe dist/semver.{{tag}}_x86_64-pc-windows-gnu.exe
    cp target/x86_64-unknown-linux-gnu/release/semver dist/semver.{{tag}}_x86_64-unknown-linux-gnu
    gh release upload {{tag}} dist/semver.*