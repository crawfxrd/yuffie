#!/bin/sh
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install Rust via rustup, along with the required toolchains and programs.

# shellcheck shell=dash
# shellcheck disable=SC1091

set -Ee

# Must match the toolchain in `rust-toolchain.toml`.
RUST_TOOLCHAIN="1.78.0"
# Must match version in `justfile`.
RUST_TOOLCHAIN_NIGHTLY="nightly-2024-05-11"

if ! command -v rustup >/dev/null 2>&1; then
    if command -v rustup-init >/dev/null 2>&1; then
        rustup-init -y \
            --default-toolchain none \
            --profile minimal \
            --no-update-default-toolchain
        . "${HOME}/.cargo/env"
    fi
fi

rustup toolchain install $RUST_TOOLCHAIN \
    --target x86_64-unknown-uefi \
    --profile minimal \
    --component clippy
rustup toolchain install $RUST_TOOLCHAIN_NIGHTLY \
    --profile minimal \
    --component rustfmt

# Don't run these on CI.
if [ -z "${CI}" ]; then
    rustup toolchain install --profile minimal stable

    if ! command -v mdbook >/dev/null 2>&1; then
        cargo +stable install mdbook
    fi
fi
