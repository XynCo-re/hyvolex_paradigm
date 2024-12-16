use bevy::{
    prelude::*,
    render::{
        mesh::Mesh,
        render_asset::RenderAssetUsages,
    },
};
use crate::{
    components::{
        Mesh3d, MeshMaterial3d, Shape, Box3d, Sphere3d, Cylinder3d, Cone3d, Capsule3d, Torus3d,
        node::ShapeType,
    },
    resources::MaterialHandles,
    err::{Error, ErrorManager, ComponentError},
};

#[derive(Debug, Clone, Copy)]
pub enum MeshVariant {
    Box,
    Sphere,
    Cylinder,
    Cone,
    Capsule,
    Torus,
    Node(ShapeType),
}

impl MeshVariant {
    fn create_shape(&self, size: f32) -> Shape {
        match self {
            MeshVariant::Box => Shape::Box(Box3d {
                width: size,
                height: size,
                depth: size,
                ..Default::default()
            }),
            MeshVariant::Sphere => Shape::Sphere(Sphere3d {
                radius: size / 2.0,
                ..Default::default()
            }),
            MeshVariant::Cylinder => Shape::Cylinder(Cylinder3d {
                radius: size / 2.0,
                height: size,
                ..Default::default()
            }),
            MeshVariant::Cone => Shape::Cone(Cone3d {
                radius: size / 2.0,
                height: size,
                ..Default::default()
            }),
            MeshVariant::Capsule => Shape::Capsule(Capsule3d {
                radius: size / 2.0,
                ..Default::default()
            }),
            MeshVariant::Torus => Shape::Torus(Torus3d {
                radius: size / 2.0,
                ring_radius: size / 8.0,
                ..Default::default()
            }),
            MeshVariant::Node(shape_type) => Shape::from_node_shape(*shape_type),
        }
    }
}

pub fn create_mesh(
    variant: MeshVariant,
    size: f32,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialHandles>,
    mut commands: Commands,
    mut error_manager: ResMut<ErrorManager>,
) {
    if size <= 0.0 && !matches!(variant, MeshVariant::Node(_)) {
        error_manager.report_error(Error::Component(ComponentError::ValidationFailed(
            format!("Mesh size must be positive, got {}", size)
        )));
        return;
    }

    let shape = variant.create_shape(size);
    let mesh = shape.create_mesh();
    let mesh_handle = meshes.add(mesh);

    let mut entity = commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.node_material.clone()),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ViewVisibility::default(),
    ));

    // Add the Shape component only for node shapes
    if let MeshVariant::Node(_) = variant {
        entity.insert(shape);
    }
}

pub fn create_node_mesh(
    shape_type: ShapeType,
    meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialHandles>,
    commands: Commands,
    error_manager: ResMut<ErrorManager>,
) {
    create_mesh(
        MeshVariant::Node(shape_type),
        0.0, // Size is ignored for nodes as they use dimensions from ShapeType
        meshes,
        materials,
        commands,
        error_manager,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_mesh_creation() {
        let mut app = App::new();
        app.init_resource::<ErrorManager>();
        app.init_resource::<MaterialHandles>();
        app.init_resource::<Assets<Mesh>>();
        
        app.add_systems(Update, |
            meshes: ResMut<Assets<Mesh>>,
            materials: Res<MaterialHandles>,
            commands: Commands,
            error_manager: ResMut<ErrorManager>,
        | {
            create_mesh(
                MeshVariant::Box,
                1.0,
                meshes,
                materials,
                commands,
                error_manager,
            );
        });
        
        app.update();
    }
}