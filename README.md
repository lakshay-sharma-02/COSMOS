# COSMOS

An interactive artificial-universe simulation built with Rust and Bevy.

## Overview

COSMOS is a scientifically-grounded simulation of an artificial universe, evolving
from the early universe through galaxy formation, star systems, planetary bodies,
life, and intelligent civilizations. The user acts as a creator and observer:

- Navigate across cosmological, galactic, stellar, and planetary scales
- Inspect individual celestial objects and their physical properties
- Alter initial conditions and seed parameters
- Create species and study civilization histories
- Eventually possess individual organisms to experience their world

## Architecture

| Crate        | Purpose                                              |
|--------------|------------------------------------------------------|
| `cosmos_sim` | Pure-Rust simulation core — physics, RNG, units      |
| `cosmos_app` | Bevy application — rendering, input, UI              |

Both crates live in a Cargo workspace at the repository root. The simulation
library has **no dependency on Bevy** — it is pure Rust and can be tested in
isolation.

## Quick Start

```sh
# Run all tests
cargo test

# Type-check the application
cargo check -p cosmos_app

# Build the application
cargo build -p cosmos_app

# Run the application
cargo run -p cosmos_app
```

## Requirements

- Rust stable (verified with `rustc 1.99.1`)
- For `cosmos_app`: a working native linker (MSVC or MinGW on Windows)

## License

Dual-licensed under MIT or Apache-2.0.
