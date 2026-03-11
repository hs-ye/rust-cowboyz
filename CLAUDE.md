# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

A turn-based space-western trading game built in Rust. Players buy/sell commodities between orbiting planets to maximise profit within a fixed number of turns. Runs as a WASM single-page app in the browser (primary target) or as a CLI binary.

## Commands

### Web (primary)
```bash
trunk serve          # Dev server with hot reload at http://localhost:8080
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
```

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Architecture

The codebase compiles as both a `cdylib` (WASM) and an `rlib`, with features gating each interface:
- `web` feature → Leptos SPA (default web build via Trunk)
- `cli` feature → `clap`-based terminal interface
- `src/lib.rs` is the shared core; `src/main.rs` is the CLI entry point

### Key Modules

| Module | Purpose |
|--------|---------|
| `src/setup.rs` | World initialisation from YAML; constructs the `World` struct |
| `src/game_state.rs` | `TravelState` enum and `GameClock`; the core state machine |
| `src/player/` | `Player` struct, inventory, ship, and action handlers (buy/sell/travel/wait) |
| `src/simulation/` | Economy pricing, orbital mechanics, commodity/planet types, turn management |
| `src/ui/web.rs` | Leptos component tree (60/40 split layout) |
| `src/ui/solar_map.rs` | Canvas-based solar system visualisation via `web-sys` |
| `src/assets/` | YAML config loading and `localStorage` save/load |

### Data Flow

1. `setup.rs` loads `data/config/planets.yaml` and `data/config/goods.yaml` into a `World`
2. Player actions in `player/actions.rs` mutate `World` state
3. `simulation/economy.rs` recalculates market prices each turn: `Base × Local Multiplier × Supply Factor × Demand Factor`
4. `simulation/orbits.rs` updates planet positions: `position = current_turn % orbit_period`
5. Travel time is derived from current/destination orbital positions
6. Leptos signals drive reactive re-renders; game state serialises to `localStorage` via `serde_json`

### Travel State Machine

`TravelState` in `game_state.rs` has two variants:
- `Idle { at_planet }` — player is docked
- `InTransit { destination, arrival_turn, departure_turn }` — player is travelling

All game changes are synchronised to turn advancement via `turn_manager.rs`.

## Configuration

Game content is data-driven via YAML:
- `data/config/planets.yaml` — 7 planet types (Agricultural, Mega City, Mining, Pirate Space Station, Research Outpost, Industrial, Frontier Colony), each with AU distance and orbit period
- `data/config/goods.yaml` — 10 commodities with base prices (Water $10 → Alien Artefacts $800), with per-planet supply/demand overrides

## Design Decisions (see `design/adr/`)

- No backend — the game is entirely client-side; persistence via `localStorage` only
- Leptos CSR (client-side rendering) mode; no SSR
- Orbital mechanics are simplified to integers: positions are `u32`, not floats
- The `World` struct is the single source of truth passed to both UI and CLI

## GitHub Conventions

This project uses a multi-agent indie studio workflow. All work is tracked through GitHub.

### Git Workflow

- Branch naming: `feat/[ticket number]-[ticket name]` (e.g. `feat/105-travel-destination-panel`)
- Branch from and PR into `master`
- PRs must include `Closes #XX` or `Fixes #XX` in the body to auto-link and close the issue

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

### Project Board (hs-ye's Rust Cowboyz dev board)

Project number: **1** | Project ID: `PVT_kwHOAHpRbM4BPxw-`

Status field ID: `PVTSSF_lAHOAHpRbM4BPxw-zg-FswM`

| Status | Option ID |
|--------|-----------|
| Backlog | `f75ad846` |
| Ready | `61e4505c` |
| In progress | `47fc9ee4` |
| In review | `df73e18b` |
| Done | `98236657` |

Move tickets to the correct status as work progresses to give visibility to the whole team.

### ADRs

Architecture decisions live in `design/adr/`. Only act on ADRs with status `accepted`. Ignore `proposed`, `superseded`, or `deprecated` ADRs.

### Agent Roles

The project is developed by a team of specialised agents (prompts in `~/indie-studio-agents/agents/`):

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
