//! # cosmos_sim
//!
//! Pure Rust simulation core for the COSMOS artificial-universe project.
//!
//! This crate contains **no rendering, windowing, or UI dependencies**. It exposes
//! deterministic simulation logic that the `cosmos_app` Bevy crate consumes.
//!
//! ## Design principles
//!
//! - Deterministic seeded randomness (reproducible across runs).
//! - f64 for simulation state.
//! - Hierarchical coordinate frames with explicit unit conversions.
//! - Separation of simulation time from rendering time.
//!
//! ## Current scope
//!
//! Milestone 1: project scaffold, crate boundaries, deterministic RNG, basic
//! coordinate types, and unit tests.

pub mod rng;
pub mod units;

pub use rng::SimRng;
pub use units::{Length, Parsec, Megaparsec};
