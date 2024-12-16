use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology, render_asset::RenderAssetUsages},
};
use crate::{
    components::{Node, Mesh3d, MeshMaterial3d},
    resources::{HelixConfig, MaterialHandles},
    err::{Error, ErrorManager, ComponentError},
};

pub fn generate_helix(
    mut commands: Commands,
    config: Res<HelixConfig>,
    materials: Res<MaterialHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut error_manager: ResMut<ErrorManager>,
) {
    if config.radius <= 0.0 {
        error_manager.report_error(Error::Component(ComponentError::ValidationFailed(
            "Helix radius must be positive".into()
        )));
        return;
    }

    let mesh = match create_node_mesh(config.radius) {
        Ok(mesh) => mesh,
        Err(e) => {
            error_manager.report_error(e);
            return;
        }
    };

    let mesh_handle = meshes.add(mesh);

    commands.spawn((
        Node::default(),
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.node_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ViewVisibility::default(),
    ));
}

#[derive(Bundle)]
pub struct NodeBundle {
    pub node: Node,
    pub mesh_3d: Mesh3d,
    pub material_3d: MeshMaterial3d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
}

impl Default for NodeBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            mesh_3d: Mesh3d(Handle::default()),
            material_3d: MeshMaterial3d(Handle::default()),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct MeshBundle {
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
}

impl Default for MeshBundle {
    fn default() -> Self {
        Self {
            mesh: Mesh3d(Handle::default()),
            material: MeshMaterial3d(Handle::default()),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

fn create_node_mesh(radius: f32) -> Result<Mesh, Error> {
    if radius <= 0.0 {
        return Err(Error::Component(ComponentError::ValidationFailed(
            format!("Mesh radius must be positive, got {}", radius)
        )));
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    
    let vertices = generate_sphere_vertices(radius, 16, 16);
    let normals = generate_sphere_normals(&vertices);
    let indices = generate_sphere_indices(16, 16);
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    
    Ok(mesh)
}

fn generate_sphere_vertices(radius: f32, segments: u32, rings: u32) -> Vec<[f32; 3]> {
    let mut vertices = Vec::new();
    
    for ring in 0..=rings {
        let phi = std::f32::consts::PI * ring as f32 / rings as f32;
        for segment in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * segment as f32 / segments as f32;
            
            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.cos();
            let z = radius * phi.sin() * theta.sin();
            
            vertices.push([x, y, z]);
        }
    }
    
    vertices
}

fn generate_sphere_normals(vertices: &[[f32; 3]]) -> Vec<[f32; 3]> {
    vertices.iter()
        .map(|&[x, y, z]| {
            let length = (x * x + y * y + z * z).sqrt();
            if length > 0.0 {
                [x / length, y / length, z / length]
            } else {
                [0.0, 1.0, 0.0]
            }
        })
        .collect()
}

fn generate_sphere_indices(segments: u32, rings: u32) -> Vec<u32> {
    let mut indices = Vec::new();
    
    for ring in 0..rings {
        for segment in 0..segments {
            let current = ring * (segments + 1) + segment;
            let next = current + segments + 1;
            
            indices.extend_from_slice(&[
                current,
                next,
                current + 1,
                current + 1,
                next,
                next + 1,
            ]);
        }
    }
    
    indices
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_node_bundle() {
        let bundle = NodeBundle::default();
        assert_eq!(bundle.transform, Transform::default());
        assert_eq!(bundle.mesh_3d.0.id(), Handle::<Mesh>::default().id());
    }

    #[test]
    fn test_mesh_bundle() {
        let bundle = MeshBundle::default();
        assert_eq!(bundle.transform, Transform::default());
        assert_eq!(bundle.mesh.0.id(), Handle::<Mesh>::default().id());
    }

    #[test]
    fn test_create_node_mesh() {
        assert!(create_node_mesh(1.0).is_ok());
        assert!(create_node_mesh(-1.0).is_err());
        assert!(create_node_mesh(0.0).is_err());
    }
}
