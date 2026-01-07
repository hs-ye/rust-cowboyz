# Qwen Context for `rust-cowboyz`

This document provides context for the Qwen AI assistant to understand and effectively contribute to the `rust-cowboyz` project.

## 1. Project Overview

*   **What is it?** A single-player, turn-based space trading game in Rust. The player is a merchant trader in a solar system, buying and selling goods between orbiting planets.
*   **Core Features:** Turn-based travel between planets with simplified orbital mechanics, a dynamic economy with fluctuating prices, and a cargo/inventory system. Game data is configured via YAML files.
*   **Project Goals:** The main goal is to earn the most money in a fixed number of turns, which serves as a high score. The initial UI will be command-line based.

## 2. Tech Stack

List the primary languages, frameworks, and important libraries.

*   **Language:** Rust
*   **Key Crates:** `serde` (for serialization/deserialization), `serde_yaml` (for YAML parsing), `clap` (for command-line UI).
*   **Front end:** TBC - we will focus on backend functionality first and inteface using CLI, frontend can come later


## 3. Common Commands

This is the most important section. It tells me how to build, run, test, and maintain your project.

*   **Build:** `cargo build`
*   **Run:** `cargo run`
*   **Test:** `cargo test`
*   **Lint/Check:** `cargo clippy -- -D warnings`
*   **Format:** `cargo fmt`

## 4. Project Structure

Briefly explain the layout of the source code.

*   `src/main.rs`: Main application entry point, handles game loop.
*   `src/game_state.rs`: Manages the core game state, including player data and turn progression.
*   `src/player/`: Player-specific logic (inventory, ship).
*   `src/simulation/`: Core simulation logic, including `orbits.rs` for planetary movement and `economy.rs` for the trading system.
*   `src/ui/cli.rs`: Handles all command-line input and output.
*   `data/config/*.yaml`: Defines the game's static data (planets, goods, economy). All game balance and setup should be done here.

## 5. Coding Conventions & Style

Describe any important conventions.

*   "Please follow standard Rust idioms and conventions."
*   "All public functions and types should have clear documentation comments."
*   "Run `cargo fmt` before committing changes."

## 6. How to Contribute

Instructions for anyone (or any AI) working on the project.

*   "Ensure `cargo clippy` and `cargo test` pass before submitting changes."
*   "When adding new game data, update the corresponding YAML files in `data/config/`."