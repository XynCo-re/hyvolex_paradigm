use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use crate::resources::uni_color::UniColor;

#[derive(Component, Default)]
pub struct CameraController;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Mesh3d(pub Handle<Mesh>);

#[derive(Component)]
pub struct MeshMaterial3d(pub Handle<StandardMaterial>);

#[derive(Component)]
pub struct WindowBorder {
    pub color: UniColor,
}

impl Default for WindowBorder {
    fn default() -> Self {
        Self {
            color: UniColor::white(),
        }
    }
}

pub mod node;
pub mod connection;
pub mod generated_mesh;
pub mod magnetic_field;
pub mod particle_emitter;
pub mod shapes;

pub use node::{Node, ShapeType};
pub use connection::{Connection, Direction};
pub use generated_mesh::{GeneratedMesh, TridecahedronVariant};
pub use magnetic_field::{MagneticField, Polarity};
pub use particle_emitter::{ParticleEmitter, EmitterShape, InteractionEffect};
pub use shapes::*;
