use bevy::prelude::*;
use crate::err::{Result, ComponentError};

/// Component to mark an entity as a generated mesh
#[derive(Component)]
pub struct GeneratedMesh {
    pub variant: TridecahedronVariant,
    pub radius: f32,
    pub generated_at: f64, // timestamp
}

/// Represents different variants of the tridecahedron mesh
#[derive(Debug, Clone, Copy, Component)]
pub enum TridecahedronVariant {
    /// Standard tridecahedron with uniform scaling
    Alpha,
    /// Vertically stretched variant
    Beta,
    /// Horizontally compressed variant
    Gamma,
}

impl Default for TridecahedronVariant {
    fn default() -> Self {
        Self::Alpha
    }
}

impl From<u32> for TridecahedronVariant {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Alpha,
            1 => Self::Beta,
            2 => Self::Gamma,
            _ => Self::Alpha, // Default to Alpha for invalid values
        }
    }
}

impl GeneratedMesh {
    pub fn validate(&self) -> Result<()> {
        if self.radius <= 0.0 {
            return Err(ComponentError::ValidationFailed("Mesh radius must be positive".to_string()).into());
        }
        Ok(())
    }
}

impl Default for GeneratedMesh {
    fn default() -> Self {
        Self {
            variant: TridecahedronVariant::default(),
            radius: 1.0,
            generated_at: 0.0,
        }
    }
} 