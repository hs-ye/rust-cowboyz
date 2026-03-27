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
*   **Frontend:** Leptos (WASM SPA) — primary target; CLI available as secondary interface

## 3. Common Commands

This is the most important section. It tells me how to build, run, test, and maintain your project.

### Web (Primary)
```bash
trunk serve                    # Dev server with hot reload at http://localhost:8080
trunk build --release
```

### CLI
```bash
cargo run --features cli
cargo build --features cli --release
```

### Testing & Checking
```bash
cargo test
cargo check --features web
cargo clippy --features web
cargo fmt
```

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## 4. Project Structure

Briefly explain the layout of the source code.

| Module | Purpose |
|--------|---------|
| `src/lib.rs` | Shared core library |
| `src/main.rs` | CLI entry point |
| `src/setup.rs` | World initialisation from YAML |
| `src/game_state.rs` | `TravelState` enum and `GameClock`; core state machine |
| `src/player/` | `Player` struct, inventory, ship, action handlers (buy/sell/travel/wait) |
| `src/simulation/` | Economy pricing, orbital mechanics, commodity/planet types, turn management |
| `src/ui/web.rs` | Leptos component tree (60/40 split layout) |
| `src/ui/solar_map.rs` | Canvas-based solar system visualisation via `web-sys` |
| `src/assets/` | YAML config loading and `localStorage` save/load |
| `data/config/*.yaml` | Game static data (planets, goods, economy) |

## 5. Coding Conventions & Style

Describe any important conventions.

*   Follow standard Rust idioms and conventions
*   All public functions and types should have clear documentation comments
*   Run `cargo fmt` before committing changes
*   Ensure `cargo clippy` and `cargo test` pass before submitting changes

## 6. Architecture

*   **Data Flow:** `setup.rs` loads YAML configs → `World` struct → player actions mutate state → economy/orbits recalculate each turn → Leptos signals drive reactive re-renders
*   **Travel State Machine:** `TravelState` has `Idle { at_planet }` and `InTransit { destination, arrival_turn, departure_turn }`
*   **Persistence:** Game state serialises to `localStorage` via `serde_json`
*   **No Backend:** Entirely client-side; no server required

## 7. GitHub Conventions

This project uses a multi-agent indie studio workflow. All work is tracked through GitHub.

### Git Workflow

*   Branch naming: `feat/[ticket-number]-[ticket-name]` (e.g., `feat/105-travel-destination-panel`)
*   Branch from and PR into `master`
*   PRs must include `Closes #XX` or `Fixes #XX` in the body to auto-link and close the issue

### Issue Labels

| Label | Meaning |
|-------|---------|
| `master-backlog` | Identifies the master backlog issue (project-manager only) |
| `epic` | Epic-level issue |
| `frontend` | Frontend task → software-engineer |
| `backend` | Backend task → software-engineer |
| `ui` | UI/UX task → design-specialist |
| `dependencies` | Must be completed before other tickets it specifies |
| `tech-lead-review` | Needs technical lead input before proceeding |
| `project-manager-review` | Needs PM attention |
| `qa` | Needs QA-tester verification (not for unit tests) |
| `bug` | Bug report; reviewed by tech-lead before assignment |
| `blocking` | Blocks other work; escalate immediately |
| `user-input-required` | A user decision is required |

### Project Board

Project number: **1** | Project ID: `PVT_kwHOAHpRbM4BPxw-`

Status field ID: `PVTSSF_lAHOAHpRbM4BPxw-zg-FswM`

| Status | Option ID |
|--------|-----------|
| Backlog | `f75ad846` |
| Ready | `61e4505c` |
| In progress | `47fc9ee4` |
| In review | `df73e18b` |
| Done | `98236657` |

Move tickets to the correct status as work progresses.

### ADRs

Architecture decisions live in `design/adr/`. Only act on ADRs with status `accepted`. Ignore `proposed`, `superseded`, or `deprecated` ADRs.

### Agent Roles

| Agent | Responsibility |
|-------|---------------|
| `project-manager` | Orchestrates the team, manages the master backlog, escalates to user |
| `software-architect` | Creates and maintains ADRs for technical decisions |
| `technical-lead` | Breaks epics into tickets, reviews PRs, merges to master |
| `software-engineer` | Implements features and fixes from tickets |
| `game-designer` | Authors game design ADRs |
| `design-specialist` | Owns UI/UX work (`ui`-labelled tickets) |
| `qa-tester` | Verifies completed work, raises bug tickets |
| `playtester` | Playwright-based web UI testing of the WASM build |