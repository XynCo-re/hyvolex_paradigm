use bevy::{
    prelude::*,
    pbr::StandardMaterial,
};
use crate::{
    err::{Error, ErrorManager, ResourceError},
    resources::uni_color::UniColor,
};

#[derive(Resource)]
pub struct MaterialHandles {
    pub node_material: Handle<StandardMaterial>,
    pub connection_material: Handle<StandardMaterial>,
    pub highlight_material: Handle<StandardMaterial>,
}

impl Default for MaterialHandles {
    fn default() -> Self {
        Self {
            node_material: Handle::default(),
            connection_material: Handle::default(),
            highlight_material: Handle::default(),
        }
    }
}

pub fn create_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_handles: ResMut<MaterialHandles>,
    mut error_manager: ResMut<ErrorManager>,
) {
    // Create default material
    let base_color = UniColor::srgb(0.9, 0.9, 0.9);
    let emissive = UniColor::srgb(0.1, 0.1, 0.1);
    
    let material = StandardMaterial {
        base_color: base_color.as_bevy_color(),
        emissive: emissive.as_linear_rgba(),
        ..default()
    };

    material_handles.node_material = materials.add(material);

    // Create connection material
    let connection_color = UniColor::srgb(0.3, 0.5, 0.8).with_alpha(0.8);
    let connection_material = StandardMaterial {
        base_color: connection_color.as_bevy_color(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    };

    material_handles.connection_material = materials.add(connection_material);

    // Create highlight material
    let highlight_color = UniColor::srgb(1.0, 0.8, 0.0);
    let highlight_emissive = UniColor::srgb(0.8, 0.6, 0.0);
    
    let highlight_material = StandardMaterial {
        base_color: highlight_color.as_bevy_color(),
        emissive: highlight_emissive.as_linear_rgba(),
        ..default()
    };

    material_handles.highlight_material = materials.add(highlight_material);

    error_manager.report_with_recovery(
        Error::Resource(ResourceError::LoadFailed("Materials initialized".into())),
        "Using default materials"
    );
} 