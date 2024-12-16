// src/lib.rs

#![allow(non_snake_case)]

//! # Critical System Configuration Notes & Warnings
//! 
//! ## STRICT REQUIREMENTS:
//! 1. DO NOT use .in_set() on system tuples directly - it causes trait bound errors
//!    - INCORRECT: app.add_systems(Update, (sys1, sys2).in_set(Set))
//!    - CORRECT:   Register each system individually with its set
//! 
//! 2. DO NOT use .into_config() - it's not compatible with Bevy 0.15.0
//!    - INCORRECT: app.add_systems(Update, sys1.into_config())
//!    - CORRECT:   Use direct configuration methods
//! 
//! 3. DO NOT combine multiple system configuration modifiers in a single chain
//!    - INCORRECT: app.add_systems(Update, sys1.in_set(Set1).after(Set2))
//!    - CORRECT:   Configure sets separately from system registration
//! 
//! 4. System sets MUST be configured separately from system registration
//!    - INCORRECT: Mixing set configuration with system addition
//!    - CORRECT:   First configure all sets, then register systems
//! 
//! 5. Keep system registration simple - avoid complex chaining
//!    - INCORRECT: Nested configuration chains
//!    - CORRECT:   One configuration per line
//! 
//! ## Additional Guidelines:
//! - Always register systems individually to prevent trait bound errors
//! - Configure set relationships explicitly using .before() and .after()
//! - Initialize resources separately to maintain clarity
//! - Add plugins individually rather than in groups
//! - Keep startup systems separate from update systems
//! 
//! ## Common Pitfalls:
//! 1. Tuple system registration with sets causes compile errors
//! 2. Chained configurations can lead to unexpected behavior
//! 3. Mixed set configuration and system registration causes conflicts
//! 4. Complex system chains make debugging difficult
//! 5. Grouped plugin registration can mask initialization issues

pub mod components;
pub mod err;
pub mod resources;
pub mod systems;

use bevy::{
    prelude::*,
    app::PluginGroup,
    log::LogPlugin,
};
use bevy_hanabi::HanabiPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_mod_outline::OutlinePlugin;

use crate::{
    resources::{HelixConfig, MaterialHandles},
    systems::{
        setup::{setup_materials, setup_camera, setup_scene},
        intersections::check_intersections,
        magnetic::update_magnetic_fields,
        node_visuals::update_node_visuals,
        particles::update_particles,
        generation::generate_helix,
    },
    err::{ErrorManager, error_check_system},
};

// Re-export error types from err module
pub use crate::err::{Error, ComponentError, ResourceError, SystemError};

#[derive(Resource)]
pub struct AnimationState {
    pub paused: bool,
    pub speed: f32,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            paused: false,
            speed: 1.0,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum HyvoGridSet {
    Setup,
    Physics,
    Rendering,
    ErrorHandling,
}

pub struct HyvoGrid;

impl Default for HyvoGrid {
    fn default() -> Self {
        Self
    }
}

impl Plugin for HyvoGrid {
    fn build(&self, app: &mut App) {
        // SAFETY: Core plugins must be configured individually to prevent initialization conflicts
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=info".to_string(),
            ..default()
        }));

        // SAFETY: Third-party plugins added individually to ensure proper initialization order
        app.add_plugins(HanabiPlugin);
        app.add_plugins(TweeningPlugin);
        app.add_plugins(OutlinePlugin);

        // SAFETY: Resources must be initialized separately to maintain clear dependency chains
        app.init_resource::<AnimationState>();
        app.init_resource::<MaterialHandles>();
        app.init_resource::<HelixConfig>();
        app.init_resource::<ErrorManager>();

        // SAFETY: System sets must be configured before any system registration
        app.configure_sets(Update, HyvoGridSet::Setup);
        app.configure_sets(Update, HyvoGridSet::Physics);
        app.configure_sets(Update, HyvoGridSet::Rendering);
        app.configure_sets(Update, HyvoGridSet::ErrorHandling);
        
        // SAFETY: Set ordering must be explicit to prevent race conditions
        app.configure_sets(Update, HyvoGridSet::Setup.before(HyvoGridSet::Physics));
        app.configure_sets(Update, HyvoGridSet::Physics.before(HyvoGridSet::Rendering));
        app.configure_sets(Update, HyvoGridSet::Rendering.before(HyvoGridSet::ErrorHandling));

        // SAFETY: Startup systems registered individually to prevent initialization order issues
        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, setup_materials);
        app.add_systems(Startup, setup_scene);

        // SAFETY: Physics systems must be registered individually with set assignment
        // DO NOT combine into tuple to avoid trait bound errors
        app.add_systems(Update, generate_helix.in_set(HyvoGridSet::Physics));
        app.add_systems(Update, check_intersections.in_set(HyvoGridSet::Physics));
        app.add_systems(Update, update_magnetic_fields.in_set(HyvoGridSet::Physics));

        // SAFETY: Rendering systems must be registered individually with set assignment
        // DO NOT combine into tuple to avoid trait bound errors
        app.add_systems(Update, update_node_visuals.in_set(HyvoGridSet::Rendering));
        app.add_systems(Update, update_particles.in_set(HyvoGridSet::Rendering));

        // SAFETY: Error handling system must run after all other systems
        app.add_systems(Update, error_check_system.in_set(HyvoGridSet::ErrorHandling));
    }
}

pub fn run() {
    info!("Starting HyvoGrid application...");
    
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,hyvolex_paradigm=debug".to_string(),
            ..default()
        }))
        .add_plugins(HyvoGrid::default())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_state() {
        let state = AnimationState::default();
        assert!(!state.paused);
        assert_eq!(state.speed, 1.0);
    }
}
