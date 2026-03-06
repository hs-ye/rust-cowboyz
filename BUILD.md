# Building and Running Rust Cowboyz

This guide explains how to build and run the Rust Cowboyz game locally for development and playtesting.

## Prerequisites

Before building the project, ensure you have the following installed:

### 1. Rust Toolchain

Install Rust using [rustup](https://rustup.rs/):

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### 2. WebAssembly Target

The game compiles to WebAssembly for the web version. Add the wasm32 target:

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Trunk

Trunk is the recommended tool for building and serving WASM web apps. Install it:

```bash
# Install Trunk
cargo install trunk

# Verify installation
trunk --version
```

## Building the Web/WASM Version

### Development Build

For development with faster builds (less optimization):

```bash
trunk build
```

This creates the web assets in the `dist/` directory.

### Production Build

For a production-optimized build:

```bash
trunk build --release
```

This produces optimized WASM and minified assets in `dist/`.

## Running the Development Server

Start a local development server with hot-reload support:

```bash
trunk serve
```

This will:
- Compile the WASM
- Start a local web server
- Watch for file changes and rebuild automatically

The server runs on:
- **URL**: http://0.0.0.0:8080
- **Port**: 8080 (configured in Trunk.toml)

## Accessing the Game

Once the server is running, open your browser and navigate to:

```
http://localhost:8080
```

## Development Workflow

### Typical Development Cycle

1. **Start the development server**:
   ```bash
   trunk serve
   ```

2. **Make code changes** in `src/`

3. **View changes** - Trunk automatically rebuilds on file changes. Refresh your browser to see updates.

4. **Check for errors** - Monitor the terminal for compilation errors

### Running CLI Version (Alternative)

The project also supports a CLI mode. To run without the web UI:

```bash
# Build and run CLI version
cargo run --features cli

# Or with release optimization
cargo run --features cli --release
```

## Common Issues and Solutions

### Issue: "wasm32-unknown-unknown target not found"

**Solution**: Install the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

### Issue: "trunk: command not found"

**Solution**: Install Trunk:
```bash
cargo install trunk
```

### Issue: Build fails with "feature 'web' is required"

**Solution**: Ensure you're building with the web feature (Trunk.toml already includes this):
```bash
trunk build
# Or explicitly:
trunk build --features web
```

### Issue: Browser shows "Loading..." but nothing happens

**Solution**:
1. Check browser console for errors (F12 → Console)
2. Ensure you're using a modern browser with WASM support
3. Try clearing browser cache and refreshing

### Issue: Port 8080 already in use

**Solution**: Change the port in `Trunk.toml` or kill the process using port 8080:
```bash
# Find process using port 8080
lsof -i :8080

# Kill the process (replace PID with actual process ID)
kill -9 <PID>
```

### Issue: Slow build times

**Solution**:
- Use `trunk serve` for development (faster rebuilds)
- Use release mode only when needed: `trunk build --release`
- Consider using `cargo-watch` for faster incremental builds

## Project Structure

```
rust-cowboyz/
├── src/              # Rust source code
├── public/           # Static assets (copied to dist/)
├── dist/             # Build output (generated)
├── index.html        # HTML template for web build
├── Trunk.toml        # Trunk build configuration
├── Cargo.toml        # Rust project configuration
└── BUILD.md          # This file
```

## Additional Commands

### Clean Build Artifacts

```bash
# Clean Cargo build cache
cargo clean

# Remove dist folder
rm -rf dist/
```

### Check for Errors Without Building

```bash
cargo check --features web
```

### Run Tests

```bash
cargo test
```

## Further Reading

- [Trunk Documentation](https://trunkrs.dev/)
- [Rust WASM Book](https://rustwasm.github.io/book/)
- [Leptos Framework](https://leptos.dev/) (used for web UI)