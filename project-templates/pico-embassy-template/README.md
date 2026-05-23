# Common `cargo` commands for embedded projects:

## Building & Flashing

| Command | Purpose |
|---------|---------|
| `cargo build` | Compile in debug mode |
| `cargo build --release` | Compile with optimizations |
| `cargo embed` | Build and flash to device (debug mode) |
| `cargo embed --release` | Build and flash to device (release mode) |

## Debugging

| Command | Purpose |
|---------|---------|
| `cargo embed` | Flash and start GDB debugging session |
| `cargo run` | Build and run (for native targets only) |

## Cleaning & Checking

| Command | Purpose |
|---------|---------|
| `cargo clean` | Remove build artifacts |
| `cargo check` | Check code without building |
| `cargo clippy` | Lint code for common mistakes |

## Testing

| Command | Purpose |
|---------|---------|
| `cargo test` | Run unit tests (host only) |
| `cargo test --release` | Run tests with optimizations |

## Utilities

| Command | Purpose |
|---------|---------|
| `cargo size` | Show binary size (requires `cargo-binutils`) |
| `cargo objdump` | Disassemble binary (requires `cargo-binutils`) |
| `cargo tree` | Show dependency tree |

