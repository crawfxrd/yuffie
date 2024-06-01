#!/bin/sh
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install system dependencies for building the project.

# shellcheck shell=dash
# shellcheck disable=SC1091

set -Ee

. /etc/os-release
if [ "${ID}" = "arch" ]; then
    sudo pacman -S --noconfirm \
        curl \
        just \
        rustup \
        shellcheck
elif [ "${ID}" = "debian" ] || [ "${ID}" = "ubuntu" ]; then
    sudo apt -q update
    sudo apt -q install --no-install-recommends --assume-yes \
        curl \
        just \
        rustup \
        shellcheck
elif [ "${ID}" = "fedora" ]; then
    sudo dnf install --assumeyes \
        curl \
        just \
        rustup \
        shellcheck
else
    echo "unsupported host: ${ID}"
    exit 1
fi
