# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`tae` is a Rust binary crate (edition 2024, no external dependencies yet). The entry point is `src/main.rs`.

## Commands

```bash
cargo build          # compile (debug)
cargo build --release
cargo run            # build and run
cargo test           # run all tests
cargo test <name>    # run a single test by name (substring match)
cargo clippy         # lint
cargo fmt            # format
```