// src/systems/setup.rs

use bevy::{
    prelude::*,
    pbr::StandardMaterial,
    render::{
        mesh::Mesh,
        render_resource::PrimitiveTopology,
    },
};
use crate::{
    components::{Mesh3d, MeshMaterial3d, MainCamera, WindowBorder},
    resources::{MaterialHandles, uni_color::UniColor},
    err::{Error, ErrorManager, ComponentError, ResourceError},
};

/// A marker component for our shapes
#[derive(Component)]
pub struct Shape;

pub fn setup_camera(
    mut commands: Commands,
    mut error_manager: ResMut<ErrorManager>,
) {
    let camera_position = Vec3::new(-2.0, 2.5, 5.0);
    let look_at = Vec3::ZERO;

    if camera_position.distance(look_at) < 0.1 {
        error_manager.report_error(Error::Component(ComponentError::ValidationFailed(
            "Camera position too close to look_at point".into()
        )));
        return;
    }

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(camera_position)
                .looking_at(look_at, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

pub fn setup_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_handles: ResMut<MaterialHandles>,
    _error_manager: ResMut<ErrorManager>,
) {
    // Create default material
    let base_color = UniColor::srgb(0.98, 0.96, 0.93);
    let emissive = UniColor::srgb(1.0, 1.0, 0.0);
    
    let material = StandardMaterial {
        base_color: base_color.as_bevy_color(),
        emissive: emissive.as_linear_rgba(),
        ..default()
    };

    material_handles.node_material = materials.add(material);

    // Create connection material
    let connection_color = UniColor::srgb(0.0, 0.0, 1.0).with_alpha(0.8);
    let connection_material = StandardMaterial {
        base_color: connection_color.as_bevy_color(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    };

    material_handles.connection_material = materials.add(connection_material);

    // Create highlight material
    let highlight_color = UniColor::srgb(1.0, 0.84, 0.0);
    let highlight_emissive = UniColor::srgb(1.0, 1.0, 0.0);
    
    let highlight_material = StandardMaterial {
        base_color: highlight_color.as_bevy_color(),
        emissive: highlight_emissive.as_linear_rgba(),
        ..default()
    };

    material_handles.highlight_material = materials.add(highlight_material);
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialHandles>,
    mut error_manager: ResMut<ErrorManager>,
) {
    // Create grid with connection material
    let grid_mesh = Mesh::from(shape::Plane::from_size(10.0));
    let grid_handle = meshes.add(grid_mesh);

    commands.spawn((
        Mesh3d(grid_handle),
        MeshMaterial3d(materials.connection_material.clone()),
        Transform::from_xyz(0.0, -0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ViewVisibility::default(),
    ));

    // Create origin marker with node material
    let sphere_mesh = Mesh::from(shape::UVSphere::default());
    let sphere_handle = meshes.add(sphere_mesh);

    commands.spawn((
        Mesh3d(sphere_handle),
        MeshMaterial3d(materials.node_material.clone()),
        Transform::from_scale(Vec3::splat(0.1)),
        GlobalTransform::default(),
        Visibility::default(),
        ViewVisibility::default(),
    ));
}

pub fn setup_window_border(
    mut windows: Query<&mut Window>,
    error_manager: ResMut<ErrorManager>,
) {
    let mut window = match windows.get_single_mut() {
        Ok(window) => window,
        Err(_) => {
            error_manager.report_error(Error::Resource(ResourceError::NotFound(
                "Primary window not found".into()
            )));
            return;
        }
    };

    window.decorations = false;
    window.resolution.set(1280.0, 720.0);
}

pub fn animate_window_border(
    time: Res<Time>,
    mut query: Query<&mut WindowBorder>,
) {
    let elapsed = time.elapsed_secs_f64() as f32;
    
    for mut border in query.iter_mut() {
        let t = (elapsed.sin() * 0.5 + 0.5) as f32;
        let base_color = UniColor::srgb(1.0, 0.7 - (0.2 * t), 0.0);
        border.color = base_color.with_alpha(0.8 + (t * 0.2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_camera_setup() {
        let mut app = App::new();
        app.init_resource::<ErrorManager>();
        
        app.add_systems(Startup, setup_camera);
        app.update();
        
        assert!(app.world.query::<&Camera3d>().iter(&app.world).count() == 1);
    }

    #[test]
    fn test_materials_setup() {
        let mut app = App::new();
        app.init_resource::<ErrorManager>();
        app.init_resource::<MaterialHandles>();
        app.init_resource::<Assets<StandardMaterial>>();
        
        app.add_systems(Startup, setup_materials);
        app.update();
        
        let materials = app.world.resource::<MaterialHandles>();
        assert!(materials.node_material.id() != Handle::default().id());
        assert!(materials.connection_material.id() != Handle::default().id());
        assert!(materials.highlight_material.id() != Handle::default().id());
    }
}
