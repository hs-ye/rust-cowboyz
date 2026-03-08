# 0000: Rust Cowboyz Project Principles

## Status
Accepted

## Date
2026-02-26

## Deciders
- User - project lead

## Project Principles

This is a small indie Singleplayer, space-western themed game that is primarily based on resource/commodity trading gameply with simplified orbital mechanics
- Simple turn based game, Single page Web browser based UI, no complex menus or screens.
- No login backend - state should be saved entirely in the user's browser upon loading in
- Simple gameplay: Trading is the core game play: buy low, sell high. 
    - Planets are the only locations that players can move between.
- Simple UI: Jump in and play, game systems / UI should be self-explanatory and minimal instructions / tutorials needed
- Scope: Single solar system: multiple planets orbiting a single sun. 
    - System itself can be randomised and doesn't need to (although it could) be realistic or based on any real solar system
- Key strategic challenge: Players should be trying to use the (simplified) orbital mechanics model in the game to plan the best route between planets as they orbit/rotate around the star at different speeds
- Basic ship customisation and upgrades: not aiming to build a full ship configuration system, just enough abstracted upgrades (Flight Speed, Cargo etc.)
- Language: The game UI and all text content will be in English only. No internationalization (i18n) support in the current version.
Project management: Build MVP first, get the minimum system working, don't over-engineer. Prefer simple, accessible features over complexity.

### Future extensions
This are things that that are explicitly OUT of scope in the current version. 
They may be added in a future update
- Combat system
- Multiplayer
- Lore and background stories / narrative
- Quest system
- Realistic orbital mechanics, movement to arbitratry locations in the system.

## High-Level Technical Decisions

Architecture: Single Page WebbApp, written entirely in Rust
Core code: Native/Stock Rust
Frontend UI Framework: use Leptos for web browser rendering, handling input from user
Asset Management: within bundle sent to Browser, minimal backend
Graphics: Some form of 2D sprite/pixel based graphics, TBC in another ADR

### Advantages
- Single codebase in Rust
- No complex client-server communication
- Runs entirely in browser
- Good performance via WebAssembly
- Offline capability possible
- Easy deployment (static hosting)
