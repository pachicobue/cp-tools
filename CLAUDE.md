# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `cp-tools` (cpt), a Rust-based command line tool for competitive programming. It provides test helpers and hackcase generation for competitive programming problems with support for batch, special judge, and reactive judge modes.

## Architecture

The project is organized as a Rust workspace with three main crates:

- `cpt-core/` - Main CLI program and core functionality
- `cpt-extra/` - Additional utilities and expansion commands  
- `cpt-stdx/` - Standard library extensions and utilities

### Core Structure

- `cpt-core/src/main.rs` - Entry point with CLI argument parsing using clap
- `cpt-core/src/commands/` - Command implementations split by functionality:
  - `test/` - Testing commands (batch, special, reactive)
  - `hack/` - Hackcase generation commands (batch, special, reactive)
- `cpt-core/src/judge/` - Judge implementations for different modes
- `cpt-core/src/generator.rs` - Input/output generation utilities
- `cpt-core/src/testcase.rs` - Testcase management

## Development Commands

### Build and Install
```bash
# Install main CLI
cargo install --path cpt-core

# Install additional utilities
cargo install --path cpt-extra

# Build all workspace members
cargo build --workspace
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p cpt
```

### Format and Lint
```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace
```

## CLI Usage Patterns

The tool follows a hierarchical command structure:
- `cpt test batch` (alias: `cpt t b`) - Basic batch testing
- `cpt test special` (alias: `cpt t s`) - Special judge testing  
- `cpt test reactive` (alias: `cpt t r`) - Reactive judge testing
- `cpt hack batch` (alias: `cpt h b`) - Batch hackcase generation
- `cpt hack special` (alias: `cpt h s`) - Special judge hackcase generation
- `cpt hack reactive` (alias: `cpt h r`) - Reactive judge hackcase generation

Common arguments:
- `-c` - Command to test/hack
- `-j` - Judge command
- `-i` - Input generator
- `-d` - Directory for testcases
- `--tl` - Time limit (hidden argument)