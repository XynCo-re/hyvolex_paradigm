use bevy::prelude::*;
use crate::err::{Result, ResourceError};

/// Animation state configuration
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

/// Camera configuration state
#[derive(Resource)]
pub struct CameraState {
    pub target_yaw: f32,
    pub target_pitch: f32,
    pub target_radius: f32,
    pub target_focus: Vec3,
    pub transition_time: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            target_yaw: -std::f32::consts::FRAC_PI_4,
            target_pitch: std::f32::consts::FRAC_PI_6,
            target_radius: 20.0,
            target_focus: Vec3::ZERO,
            transition_time: 0.0,
        }
    }
}

/// Configuration for HyvoGrid plugins
#[derive(Resource, Clone)]
pub struct HyvoGridConfig {
    pub enable_hanabi: bool,
    pub enable_tweening: bool,
    pub enable_outline: bool,
    pub enable_camera: bool,
}

impl Default for HyvoGridConfig {
    fn default() -> Self {
        Self {
            enable_hanabi: true,
            enable_tweening: true,
            enable_outline: true,
            enable_camera: true,
        }
    }
}

impl HyvoGridConfig {
    pub fn validate(&self) -> Result<()> {
        // Add validation logic if needed in the future
        Ok(())
    }
}

/// General simulation configuration
#[derive(Resource, Default)]
pub struct SimulationConfig {
    pub simulation_speed: f32,
    pub particle_density: f32,
}

impl SimulationConfig {
    pub fn validate(&self) -> Result<()> {
        if self.simulation_speed <= 0.0 {
            return Err(ResourceError::InvalidConfig("Simulation speed must be positive".to_string()).into());
        }
        if self.particle_density < 0.0 {
            return Err(ResourceError::InvalidConfig("Particle density cannot be negative".to_string()).into());
        }
        Ok(())
    }
} 