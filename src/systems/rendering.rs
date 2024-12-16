use bevy::prelude::*;

use crate::{
    components::{
        node::{Node, ShapeType},
        Mesh3d,
        MeshMaterial3d,
    },
    components::mesh_generator::{create_tridecahedron, TridecahedronVariant},
    resources::Materials,
    err::{Result, SystemError},
};

pub fn update_rendering_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
    query: Query<(Entity, &Node, &Transform), Added<Node>>,
) {
    for (entity, node, transform) in query.iter() {
        // Convert ShapeType to TridecahedronVariant
        let variant = match node.shape_type {
            ShapeType::Alpha => TridecahedronVariant::Alpha,
            ShapeType::Beta => TridecahedronVariant::Beta,
            ShapeType::Gamma => TridecahedronVariant::Gamma,
        };

        // Create mesh using our custom mesh generator
        let mesh = match create_tridecahedron(0.5, variant) {
            Ok(mesh) => mesh,
            Err(e) => {
                error!("Failed to create tridecahedron mesh: {}", e);
                continue;
            }
        };
        
        let mesh_handle = meshes.add(mesh);

        // Add mesh and material components
        commands.entity(entity).insert((
            Mesh3d(mesh_handle),
            MeshMaterial3d(materials.node_material.clone()),
        ));
    }
}