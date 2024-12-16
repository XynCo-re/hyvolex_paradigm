use bevy::prelude::*;
use crate::err::{Result, ResourceError};

#[derive(Resource)]
pub struct HelixConfig {
    pub radius: f32,
    pub vertical_step: f32,
    pub nodes_per_strand: u32,
    pub magnetic_field_strength: f32,
    pub particle_emission_rate: f32,
    pub interaction_radius: f32,
    pub node_radius: f32,
}

impl HelixConfig {
    pub fn validate(&self) -> Result<()> {
        if self.radius <= 0.0 {
            return Err(ResourceError::InvalidConfig("Helix radius must be positive".to_string()).into());
        }
        if self.vertical_step <= 0.0 {
            return Err(ResourceError::InvalidConfig("Vertical step must be positive".to_string()).into());
        }
        if self.nodes_per_strand == 0 {
            return Err(ResourceError::InvalidConfig("Must have at least one node per strand".to_string()).into());
        }
        if self.magnetic_field_strength <= 0.0 {
            return Err(ResourceError::InvalidConfig("Magnetic field strength must be positive".to_string()).into());
        }
        if self.particle_emission_rate < 0.0 {
            return Err(ResourceError::InvalidConfig("Particle emission rate cannot be negative".to_string()).into());
        }
        if self.interaction_radius <= 0.0 {
            return Err(ResourceError::InvalidConfig("Interaction radius must be positive".to_string()).into());
        }
        if self.node_radius <= 0.0 {
            return Err(ResourceError::InvalidConfig("Node radius must be positive".to_string()).into());
        }
        Ok(())
    }
}

impl Default for HelixConfig {
    fn default() -> Self {
        Self {
            radius: 5.0,
            vertical_step: 1.0,
            nodes_per_strand: 10,
            magnetic_field_strength: 1.0,
            particle_emission_rate: 50.0,
            interaction_radius: 2.0,
            node_radius: 0.5,
        }
    }
} 