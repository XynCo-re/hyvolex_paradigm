use bevy::prelude::*;
use bevy_hanabi::prelude::*;

/// Effects for magnetic field visualization
#[derive(Resource)]
pub struct MagneticEffects {
    pub north: Handle<EffectAsset>,
    pub south: Handle<EffectAsset>,
    pub interaction: Handle<EffectAsset>,
}

/// Effects for node visualization
#[derive(Resource)]
pub struct NodeEffects {
    pub active: Handle<EffectAsset>,
    pub highlight: Handle<EffectAsset>,
    pub pulse: Handle<EffectAsset>,
}

/// Effects for intersection visualization
#[derive(Resource)]
pub struct IntersectionEffects {
    pub collision: Handle<EffectAsset>,
    pub connection: Handle<EffectAsset>,
}

impl Default for MagneticEffects {
    fn default() -> Self {
        Self {
            north: Handle::default(),
            south: Handle::default(),
            interaction: Handle::default(),
        }
    }
}

impl Default for NodeEffects {
    fn default() -> Self {
        Self {
            active: Handle::default(),
            highlight: Handle::default(),
            pulse: Handle::default(),
        }
    }
}

impl Default for IntersectionEffects {
    fn default() -> Self {
        Self {
            collision: Handle::default(),
            connection: Handle::default(),
        }
    }
} 