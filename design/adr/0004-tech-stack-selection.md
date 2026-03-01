# 0004: Tech Stack Selection for Space-Western Trading Game

## Status
Accepted

## Date
2026-03-01

## Deciders
- User
- software-architect

## Context
We need to establish a comprehensive tech stack for the space-western trading game that leverages Rust's performance benefits while enabling browser-based deployment. The game requires sophisticated orbital calculations, turn-based mechanics, and a rich UI experience using Leptos as mandated. The chosen stack must support the single-page application architecture with client-side state management and provide the necessary tools for implementing the simplified orbital mechanics system.

The project has specific requirements:
- Browser-based deployment with WebAssembly compilation
- Client-side state persistence
- Real-time orbital position calculations
- Interactive UI with Leptos framework
- Turn-based game mechanics
- 2D graphics for solar system visualization
- Asset bundling for offline capability

## Decision
We will implement a Rust-based tech stack with the following components:

### Core Runtime Environment
- **Rust 1.80+**: Primary programming language for game logic and business logic
- **WebAssembly (WASM)**: Target compilation platform for browser deployment
- **Trunk**: Build tool for WASM applications, providing asset bundling and development server

### Web Framework and UI
- **Leptos 0.6+**: Reactive web framework for building the user interface
  - Server-Side Rendering (SSR) capability for initial page loads
  - Fine-grained reactivity for dynamic UI updates
  - Component-based architecture for reusable UI elements
  - Built-in state management for game state synchronization
- **Tailwind CSS**: Utility-first CSS framework for styling the space-western aesthetic
- **@leptos/router**: Client-side routing if additional views are needed in the future

### State Management and Data Handling
- **localStorage Web API**: Client-side state persistence for game saves
- **serde**: Serialization/deserialization of game state to/from JSON
- **serde_json**: JSON format for state persistence
- **gloo**: Web-oriented utilities for interacting with browser APIs
- **web-sys**: Low-level bindings to web APIs for direct browser interaction

### Graphics and Visualization
- **Canvas API**: HTML5 canvas for rendering the solar system visualization
- **Yewdux or Leptos Stores**: Reactive state management for UI components
- **web-sys Canvas bindings**: Direct manipulation of canvas elements from Rust

### Mathematical and Game Logic Libraries
Use any as appropriate.

### Testing and Development
- **wasm-bindgen-test**: Unit testing framework for WASM targets
- **spectral**: Integration testing for UI components
- **cargo-watch**: File watching for development iteration
- **wasm-pack**: Additional WASM tooling support

### Build and Asset Pipeline
- **Trunk**: Primary build tool for WASM applications
  - Bundles assets and dependencies
  - Development server with hot reloading
  - Optimized production builds
- **PostCSS**: CSS post-processing for Tailwind
- **ESBuild or SWC**: Optional JavaScript bundling if needed for complex assets

### Project Structure
```
rust-cowboyz/
├── Cargo.toml             # Dependencies and build configuration
├── Trunk.toml             # Trunk build configuration
├── index.html             # Main HTML entry point
├── style.css              # Global styles and Tailwind imports
├── src/
│   ├── lib.rs            # Main library exports and shared interfaces
│   ├── main.rs           # WASM entry point for web UI
│   ├── cli_main.rs       # CLI entry point (preserving existing functionality)
│   ├── game/             # Core game logic module (refactored from existing backend)
│   │   ├── mod.rs        # Game logic module interface
│   │   ├── state.rs      # Game state structures (preserving existing state management)
│   │   ├── orbital.rs    # Orbital mechanics calculations (from existing simulation/)
│   │   ├── economy.rs    # Economic calculations (from existing simulation/)
│   │   ├── player.rs     # Player management (refactored from existing player/)
│   │   ├── travel.rs     # Travel and movement logic (from existing simulation/)
│   │   └── turn.rs       # Turn management system
│   ├── api/              # API layer for frontend-backend communication
│   │   ├── mod.rs        # API module interface
│   │   ├── game_state.rs # Game state API endpoints
│   │   ├── market.rs     # Market data API endpoints
│   │   └── travel.rs     # Travel/trade API endpoints
│   ├── ui/               # Web UI module using Leptos
│   │   ├── mod.rs        # UI module interface
│   │   ├── app.rs        # Main application component
│   │   ├── solar_map.rs  # Solar system visualization
│   │   ├── panels.rs     # Information panels
│   │   ├── market_view.rs # Market information display
│   │   └── travel_ui.rs  # Travel interface components
│   └── utils/
│       ├── mod.rs        # Utility functions
│       ├── math.rs       # Math utilities
│       ├── storage.rs    # Local storage helpers
│       └── serialization.rs # Game state serialization (for browser persistence)
├── assets/               # Static assets (sprites, sounds, etc.)
│   ├── config/           # Configuration files (preserving existing config loader)
│   └── data/             # Game data files
└── public/               # Public assets served directly
```

This structure maintains compatibility with existing backend functionality while creating a clear pathway for the Leptos frontend implementation. The core game logic is preserved in the `game/` module with clear interfaces for both the existing CLI and new web UI.

### Key Technical Specifications

**Orbital Calculations Module**:
- Implement the turn-based orbital mechanics defined in ADR #0002
- Use integer-based calculations for planet positions
- Provide functions to calculate travel times between planets
- Update planet positions per turn efficiently
- Migrate existing orbital calculations from current simulation module to new structure

**State Management**:
- Centralized game state using Leptos reactive primitives
- Serializable game state for localStorage persistence
- Reactive updates for UI components when game state changes
- Efficient diffing to minimize unnecessary re-renders
- Maintain compatibility with existing state structures from current implementation

**API Layer**:
- Create clean interfaces between game logic and UI layers
- Provide endpoints for game state, market data, and travel/trade operations
- Enable both web UI and CLI access to core game functionality
- Preserve existing CLI functionality while adding web UI support

**UI Components**:
- Solar system map component with interactive canvas
- Planet information panels
- Market price displays
- Ship status indicators
- Navigation controls
- Turn advancement buttons

### Implementation Approach
1. **Phase 1**: Refactor existing codebase to new structure, preserving CLI functionality
2. **Phase 2**: Create API layer to expose game logic to web UI
3. **Phase 3**: Set up basic Leptos application structure with Trunk
4. **Phase 4**: Implement core game state and orbital mechanics integration
5. **Phase 5**: Create the solar system visualization component
6. **Phase 6**: Add UI panels and information displays
7. **Phase 7**: Implement save/load functionality with localStorage
8. **Phase 8**: Polish UI with space-western aesthetic styling

### Risk Mitigation Strategies
- **Performance**: Profile WASM compilation and optimize critical paths for orbital calculations
- **Browser Compatibility**: Test across major browsers for consistent canvas rendering
- **Memory Management**: Monitor memory usage with complex orbital calculations and game states
- **Learning Curve**: Invest time in Leptos documentation and examples to ensure efficient development
- **Refactoring Risk**: Maintain backward compatibility during transition to minimize disruption

## Consequences

### Positive
- Leverages Rust's performance for game state calculations
- Single codebase for both game logic and UI
- Excellent performance via WebAssembly compilation
- Type-safe UI development with Leptos' strongly-typed components
- Client-side state persistence enables offline play
- Modern web standards with excellent browser compatibility
- Potential for easy expansion to more complex features later

### Negative
- Learning curve for team members unfamiliar with Leptos framework
- Larger initial bundle size due to WASM runtime
- Potential performance bottlenecks with complex orbital calculations if not optimized
- Limited debugging capabilities compared to traditional web frameworks
- Dependency on bleeding-edge web technologies that may change rapidly

## References
- [Project Principles ADR #0000](./0000-project-principles.md)
- [General Gameplay Scenario ADR #0001](./0001-general-gameplay-scenario.md)
- [Movement Mechanics System ADR #0002](./0002-movement-mechanics-system.md)
- [Web UI View ADR #0003](./0003-web-ui-view.md)
- [Leptos Documentation](https://leptos.dev/)
- [Trunk Documentation](https://trunkrs.dev/)