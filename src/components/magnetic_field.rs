use bevy::prelude::*;
use crate::err::{Result, ComponentError};
use crate::resources::uni_color::UniColor;

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct MagneticField {
    pub strength: f32,
    pub polarity: Polarity,
    pub orientation: f32,
    pub interaction_radius: f32,
    pub particle_emission_rate: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Polarity {
    North,
    South,
}

impl Default for MagneticField {
    fn default() -> Self {
        Self {
            strength: 1.0,
            polarity: Polarity::North,
            orientation: 0.0,
            interaction_radius: 2.0,
            particle_emission_rate: 10.0,
        }
    }
}

impl MagneticField {
    /// Calculate base interaction strength between two fields without considering distance
    pub fn calculate_base_interaction(&self, other: &MagneticField) -> Result<f32> {
        if self.strength <= 0.0 || other.strength <= 0.0 {
            return Err(ComponentError::InvalidState("Magnetic field strength must be positive".to_string()).into());
        }

        let interaction_strength = if self.polarity == other.polarity {
            -self.strength * other.strength // Repulsion
        } else {
            self.strength * other.strength  // Attraction
        };
        
        Ok(interaction_strength)
    }

    pub fn get_color(&self) -> Color {
        match self.polarity {
            Polarity::North => UniColor::srgb(0.0, 0.5, 1.0).as_bevy_color(), // Blue for North
            Polarity::South => UniColor::srgb(1.0, 0.2, 0.0).as_bevy_color(), // Red for South
        }
    }

    pub fn get_emission_rate(&self) -> Result<f32> {
        if self.particle_emission_rate <= 0.0 {
            return Err(ComponentError::InvalidState("Particle emission rate must be positive".to_string()).into());
        }
        if self.strength <= 0.0 {
            return Err(ComponentError::InvalidState("Field strength must be positive for particle emission".to_string()).into());
        }
        Ok(self.particle_emission_rate * self.strength)
    }

    pub fn validate(&self) -> Result<()> {
        if self.strength <= 0.0 {
            return Err(ComponentError::ValidationFailed("Field strength must be positive".to_string()).into());
        }
        if self.interaction_radius <= 0.0 {
            return Err(ComponentError::ValidationFailed("Interaction radius must be positive".to_string()).into());
        }
        if self.particle_emission_rate < 0.0 {
            return Err(ComponentError::ValidationFailed("Particle emission rate cannot be negative".to_string()).into());
        }
        Ok(())
    }
} 