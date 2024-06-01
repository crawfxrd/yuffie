# SPDX-License-Identifier: BSD-2-Clause-Patent
# SPDX-FileCopyrightText: 2024 System76, Inc.

nightly := "nightly-2024-05-11"

# List available recipes
@help:
    just --list

# Build the project
@build *args='':
    cargo build {{args}}

# Generate API documentation
@doc *args='':
    cargo doc --workspace --document-private-items --no-deps {{args}}

# Generate mdBook documentation
@book *args='':
    mdbook build docs/ {{args}}

# Run clippy
@clippy *args='':
    cargo clippy --all-features {{args}}

# Run rustfmt
@fmt *args='':
    cargo +{{nightly}} fmt --all {{args}}

# Run tests
@test *args='':
    cargo test --target x86_64-unknown-linux-gnu --no-default-features {{args}}

# Remove build artifacts
@clean:
    cargo clean
    rm -f vendor
