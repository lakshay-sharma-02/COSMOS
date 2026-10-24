//! # cosmos_app
//!
//! Bevy application for the COSMOS universe simulation.
//!
//! This crate is the rendering and UI layer. All simulation logic lives in
//! the `cosmos_sim` crate. This crate consumes simulation state and presents
//! it visually.
//!
//! ## Current scope (Milestone 1)
//!
//! A minimal Bevy application that initializes the window and displays a
//! title screen. The simulation is not yet integrated — this establishes the
//! render/simulation boundary and verifies that the Bevy toolchain compiles
//! on this system.

use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "COSMOS".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Startup, print_startup_info)
        .run();
}

/// Display a simple title text so we can verify the window renders.
fn setup(mut commands: Commands) {
    // 2D camera for rendering
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Transform::default(),
    ));

    // Title text — uses the default font bundled with Bevy.
    commands.spawn((
        Text2d::new("COSMOS — Milestone 1\n(Simulation scaffold)"),
        TextFont::from_font_size(32.0),
        TextColor(Color::WHITE),
    ));
}

/// Log confirmation that the app initialized correctly.
fn print_startup_info() {
    println!("COSMOS app initialized. Window: 1280x720");
    println!("cosmos_sim version: {}", env!("CARGO_PKG_VERSION"));
}
