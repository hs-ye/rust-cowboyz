# Rust Cowboyz - Space Trading Game

A single-player, turn-based space trading game built with Rust and WebAssembly. Play as a merchant trader in a solar system, buying and selling commodities between orbiting planets.

## Quick Start

### Prerequisites

- **Rust** - Install from https://rustup.rs/
- **Trunk** - Build tool for WASM apps: `cargo install trunk`
- **WASM target** - `rustup target add wasm32-unknown-unknown`

### Run the Game

```bash
# Start the development server
trunk serve
```

Then open your browser to **http://localhost:8080**

## How to Play

### Objective
Earn the most money in a fixed number of turns by trading commodities between planets.

### Gameplay
1. **Travel** - Select a destination planet to travel to
2. **Buy** - Purchase commodities at low prices
3. **Sell** - Sell your cargo at higher prices on other planets
4. **Repeat** - Optimize your trading routes to maximize profit

### Tips
- Different planet types produce and demand different commodities
- Prices fluctuate based on supply and demand
- Watch for market events that can affect prices
- Plan your routes carefully - travel takes turns!

## Building from Source

### Development
```bash
trunk serve
```

### Production
```bash
trunk build --release
```

### CLI Version
```bash
cargo run --features cli
```

## Project Structure

```
├── src/              # Game source code
├── design/adr/       # Architecture decision records
├── data/config/      # Game configuration (YAML)
├── public/           # Static assets
└── dist/             # Build output
```

## More Information

See [BUILD.md](./BUILD.md) for detailed build instructions and troubleshooting.