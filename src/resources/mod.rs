mod config;
mod effects;
mod helix_config;
mod materials;
pub mod uni_color;

use bevy::prelude::*;
use bevy_hanabi::EffectAsset;

pub use config::{
    AnimationState,
    CameraState,
    HyvoGridConfig,
    SimulationConfig,
};

pub use effects::{
    MagneticEffects,
    NodeEffects,
    IntersectionEffects,
};

pub use helix_config::HelixConfig;
pub use materials::{MaterialConfig, Materials, MaterialHandles};
pub use uni_color::{UniColor, MaterialColors};

// Re-export common types
pub use bevy::pbr::StandardMaterial;

#[derive(Resource)]
pub struct Effects {
    pub collision: Handle<EffectAsset>,
    pub connection: Handle<EffectAsset>,
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            collision: Handle::default(),
            connection: Handle::default(),
        }
    }
}

// Add color conversion helpers
pub mod color_helpers {
    use super::uni_color::UniColor;
    use bevy::render::color::Color;

    pub fn to_linear_rgba(color: UniColor) -> Color {
        color.as_linear_rgba()
    }

    pub fn to_bevy_color(color: UniColor) -> Color {
        color.as_bevy_color()
    }

    pub fn to_vec4(color: UniColor) -> bevy_hanabi::prelude::Vec4 {
        color.as_vec4()
    }
}