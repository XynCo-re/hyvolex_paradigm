use bevy::{
    prelude::*,
    render::primitives::Aabb,
};
use crate::{
    components::{Direction, MeshMaterial3d},
    resources::{MaterialHandles, uni_color::UniColor},
    err::{Error, ErrorManager, ComponentError},
};

#[derive(Component)]
pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

pub fn check_intersections(
    mut commands: Commands,
    query: Query<(&Transform, &Intersection)>,
    materials: Res<MaterialHandles>,
    mut error_manager: ResMut<ErrorManager>,
) {
    if let Err(e) = check_intersections_internal(&query, &materials, &mut commands) {
        error_manager.report_error(e);
    }
}

fn check_intersections_internal(
    query: &Query<(&Transform, &Intersection)>,
    materials: &Res<MaterialHandles>,
    commands: &mut Commands,
) -> Result<(), Error> {
    for (transform, intersection) in query.iter() {
        let point = transform.translation + intersection.point;
        let normal = transform.rotation * intersection.normal;

        // Create visual marker at intersection point
        commands.spawn((
            Mesh3d(Handle::<Mesh>::default()),
            MeshMaterial3d(materials.node_material.clone()),
            Transform::from_translation(point)
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, normal.normalize())),
            GlobalTransform::default(),
            Visibility::default(),
            ViewVisibility::default(),
        ));
    }

    Ok(())
}

pub fn draw_intersection_gizmos(
    query: Query<(&Transform, &Intersection)>,
    mut gizmos: Gizmos,
) {
    let point_color = UniColor::srgb(1.0, 0.3, 0.3);
    let normal_color = UniColor::srgb(0.3, 1.0, 0.3);

    for (transform, intersection) in query.iter() {
        let point = transform.translation + intersection.point;
        let normal = transform.rotation * intersection.normal;

        if point_color.is_valid() && normal_color.is_valid() {
            gizmos.sphere(point, 0.05, point_color.as_bevy_color());
            gizmos.arrow(point, point + normal.normalize() * 0.5, normal_color.as_bevy_color());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_intersection_creation() {
        let intersection = Intersection {
            point: Vec3::new(1.0, 0.0, 0.0),
            normal: Vec3::Y,
            distance: 1.0,
        };

        assert_eq!(intersection.point.x, 1.0);
        assert_eq!(intersection.normal, Vec3::Y);
        assert_eq!(intersection.distance, 1.0);
    }

    #[test]
    fn test_intersection_system() {
        let mut app = App::new();
        app.init_resource::<ErrorManager>();
        app.init_resource::<MaterialHandles>();
        
        app.add_systems(Update, check_intersections);
        app.update();
    }
} 