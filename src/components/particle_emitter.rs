use bevy::prelude::*;
use crate::err::{Result, Error};
use crate::resources::uni_color::UniColor;

/// Configuration for different types of particle emitters
#[derive(Debug, Clone)]
pub enum EmitterShape {
    Sphere { radius: f32 },
    Cone { height: f32, radius: f32 },
    Box { size: Vec3 },
}

/// Component for configurable particle emitters
#[derive(Component, Debug)]
pub struct ParticleEmitter {
    pub active: bool,
    pub lifetime: f32,
    pub elapsed: f32,
    pub shape: EmitterShape,
    pub polarity: bool, // true for north, false for south
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            active: true,
            lifetime: 5.0,
            elapsed: 0.0,
            shape: EmitterShape::Sphere { radius: 1.0 },
            polarity: true,
        }
    }
}

impl ParticleEmitter {
    pub fn new(shape: EmitterShape, lifetime: f32, polarity: bool) -> Self {
        Self {
            active: true,
            lifetime,
            elapsed: 0.0,
            shape,
            polarity,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn try_update(&mut self, dt: f32) -> Result<()> {
        if !self.active {
            return Ok(());
        }

        self.elapsed += dt;
        if self.elapsed >= self.lifetime {
            self.active = false;
        }

        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        !self.active || self.elapsed >= self.lifetime
    }

    pub fn validate(&self) -> Result<()> {
        if self.lifetime <= 0.0 {
            return Err(Error::custom("Particle emitter lifetime must be positive"));
        }
        Ok(())
    }

    pub fn get_polarity(&self) -> bool {
        self.polarity
    }
}

/// Component to track particle effects associated with specific interactions
#[derive(Component)]
pub struct InteractionEffect {
    pub source: Entity,
    pub target: Entity,
    pub strength: f32,
}

/// A single particle in the particle system
#[derive(Component)]
pub struct Particle {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub size: f32,
    pub color: UniColor,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            lifetime: 1.0,
            max_lifetime: 1.0,
            velocity: Vec3::ZERO,
            acceleration: Vec3::new(0.0, -9.81, 0.0),
            size: 0.1,
            color: UniColor::white(),
        }
    }
}

// Add helper methods for particle color manipulation
impl Particle {
    pub fn with_color(mut self, color: UniColor) -> Self {
        self.color = color;
        self
    }

    pub fn fade_out(&mut self) {
        let progress = self.lifetime / self.max_lifetime;
        self.color = self.color.with_alpha(progress);
    }

    pub fn interpolate_color(&mut self, start: UniColor, end: UniColor) {
        let progress = 1.0 - (self.lifetime / self.max_lifetime);
        let start_vec = start.as_vec4();
        let end_vec = end.as_vec4();
        let interpolated = start_vec + (end_vec - start_vec) * progress;
        self.color = UniColor::srgba(
            interpolated.x,
            interpolated.y,
            interpolated.z,
            interpolated.w
        );
    }
} 