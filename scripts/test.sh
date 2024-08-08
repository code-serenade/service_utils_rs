#!/bin/zsh

# Get the directory where the script is located
SCRIPT_DIR=$(dirname "$0")

# Get the project root directory (assuming the project root is the parent directory of the script's directory)
PROJECT_ROOT=$(realpath "$SCRIPT_DIR/..")

# Change to the project root directory
cd "$PROJECT_ROOT" || exit

# Run cargo tests with the jwt feature enabled
cargo test --all-features
