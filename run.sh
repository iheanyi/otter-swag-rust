#!/bin/bash
# Run Otter Swag with proper library paths for macOS

set -e

# Set library path for homebrew SDL2 and SDL2_mixer
export LIBRARY_PATH="$(brew --prefix sdl2)/lib:$(brew --prefix sdl2_mixer)/lib:$LIBRARY_PATH"

# Build and run
cargo run --release "$@"
