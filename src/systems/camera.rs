use bevy::{
    prelude::*,
    input::keyboard::KeyCode,
};
use crate::{
    components::MainCamera,
    err::{Error, ErrorManager, ComponentError},
};

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

pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut error_manager: ResMut<ErrorManager>,
) {
    let mut camera_transform = match query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => {
            error_manager.report_error(Error::Component(ComponentError::NotFound(
                "Camera not found".into()
            )));
            return;
        }
    };

    let mut movement = Vec3::ZERO;
    let speed = 5.0;
    let delta = time.delta_seconds();

    if keyboard.pressed(KeyCode::W) {
        movement.z -= speed * delta;
    }
    if keyboard.pressed(KeyCode::S) {
        movement.z += speed * delta;
    }
    if keyboard.pressed(KeyCode::A) {
        movement.x -= speed * delta;
    }
    if keyboard.pressed(KeyCode::D) {
        movement.x += speed * delta;
    }
    if keyboard.pressed(KeyCode::Q) {
        movement.y -= speed * delta;
    }
    if keyboard.pressed(KeyCode::E) {
        movement.y += speed * delta;
    }

    camera_transform.translation += movement;
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
} 