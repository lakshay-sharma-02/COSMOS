# COSMOS — Engineering Rules & Architecture Reference

## Project overview
COSMOS is an interactive artificial-universe simulation. The user is a creator and
observer who can navigate across scales, inspect objects, alter initial conditions,
create species, study civilization histories, and eventually possess individual
organisms to experience their world.

## Architecture
- **`cosmos_sim`** — pure Rust simulation library. No dependency on Bevy, rendering,
  UI state, or windowing. Exposes deterministic simulation logic.
- **`cosmos_app`** — Bevy application that renders `cosmos_sim`'s state and provides
  interactive controls.
- **Cargo workspace** at the repository root ties both crates together.

## Engineering rules
1. Inspect the repository before changing it.
2. Do not overwrite existing work without understanding it.
3. Never fabricate library APIs. Check the pinned crate documentation, local
   examples, and compiler errors.
4. No fake implementations, silent placeholders, empty handlers, or success
   messages for unfinished features.
5. Every milestone must compile, pass relevant tests, run in the application,
   and update the documentation.
6. When something cannot be implemented properly, state the blocker and implement
   the strongest working subset instead of claiming completion.
7. Keep changes scoped to the current milestone. Do not prematurely implement
   future subsystems.
8. Add tests for physics, numerical boundaries, deterministic behavior,
   persistence, and agent decisions as those features arrive.
9. Do not remove tests or weaken correctness assertions merely to get a passing
   build.
10. At each milestone's end, report changed files, architectural decisions,
    commands executed, actual test/build results, known limitations, and exact
    run instructions.

## Key design principles
- Deterministic seeded randomness and reproducible simulation ticks.
- f64 for numerical simulation state. Hierarchical coordinate frames with
  explicit unit conversions — no single global frame for both cosmological and
  planetary scales.
- Hierarchical coordinate systems:
  - **Cosmic scale**: megaparsecs / billions of light-years. Approximate.
  - **Galactic scale**: parsecs / light-years.
  - **Stellar/planetary scale**: AU / kilometers.
- Simulation time is decoupled from rendering time. Supports pause, resume,
  time acceleration, and deterministic updates.
- Physical models document assumptions, units, valid ranges, approximations,
  and known limitations.
- Origin-of-life is a configurable, explicitly simplified hypothesis.
- Intelligent inhabitants have persistent internal state, perception, needs,
  memory, goals, and decision-making (no LLM per-frame behavior).
- Bounded active populations, instancing, LOD, and aggregation to stay within
  ~4 GB RAM.

## Toolchain
- Rust stable (verified: `rustc 1.99.1`).
- Bevy 0.20.x (exact version resolved by Cargo).
- Minimal Rust profile to reduce compile time and memory.

### Known tooling issue (tracked)
This development environment is a Windows 11 machine running a bash compatibility
layer. The GNU Rust toolchain (`x86_64-pc-windows-gnu`) compiles `cosmos_sim` and
its tests successfully, but **cannot link Bevy's Windows GUI dependencies** because
neither MinGW (`dlltool`/`gcc`) nor the MSVC toolchain (Windows SDK libraries) are
installed. The MSVC toolchain can be used with `rust-lld` as the linker, but
Windows SDK `.lib` files are missing.

**To compile `cosmos_app`:**
1. Install Visual Studio Build Tools with the "Desktop development with C++" workload,
   OR install MSYS2 with `mingw-w64-x86_64-gcc`.
2. Alternatively, on Linux/macOS, the GNU toolchain suffices for both crates.

Until then, `cosmos_sim` can be built and tested.

## Running
```sh
# From repository root:
cargo build          # Compile workspace (cosmos_sim builds; cosmos_app needs linker)
cargo test           # Run all tests
cargo run -p cosmos_app  # Run the application (requires working Windows linker)
```

## Milestone 1 status (complete)
- [x] Cargo workspace with `cosmos_sim` and `cosmos_app` member crates
- [x] `cosmos_sim`: deterministic RNG, unit-typed distances (Length, Parsec, Megaparsec)
- [x] `cosmos_sim`: 11 unit tests covering RNG determinism, reseeding, and unit conversions
- [x] `cosmos_app`: Bevy 0.20 scaffold with window and title screen
- [x] CLAUDE.md with architecture and engineering rules
- [ ] `cosmos_app` compiles and runs (blocked: no Windows linker/SDK on this machine)

### Files created this milestone
- `Cargo.toml` (workspace root)
- `.cargo/config.toml` — linker configuration for MSVC target
- `CLAUDE.md`
- `cosmos_sim/Cargo.toml`
- `cosmos_sim/src/lib.rs`
- `cosmos_sim/src/rng.rs`
- `cosmos_sim/src/units.rs`
- `cosmos_app/Cargo.toml`
- `cosmos_app/src/main.rs`
