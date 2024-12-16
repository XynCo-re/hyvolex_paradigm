use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::{
    components::{
        Node,
        MagneticField,
        Polarity,
        particle_emitter::{ParticleEmitter, EmitterShape, InteractionEffect, Particle}
    },
    err::{Result, Error, ComponentError, ErrorManager},
    resources::uni_color::{UniColor, presets::particle_gradient},
};
use bevy::math::Vec4;

#[derive(Component)]
#[derive(Resource)]
pub struct MagneticEffects {
    pub north: Handle<EffectAsset>,
    pub south: Handle<EffectAsset>,
    pub fallback: Handle<EffectAsset>,
}

/// Creates a position modifier based on the emitter shape
fn create_position_modifier(shape: &EmitterShape, writer: &ExprWriter) -> Box<dyn Modifier> {
    match shape {
        EmitterShape::Sphere { radius } => {
            let init_pos = SetPositionSphereModifier {
                center: writer.lit(Vec3::ZERO).expr(),
                radius: writer.lit(*radius).expr(),
                dimension: ShapeDimension::Volume,
            };
            Box::new(init_pos)
        },
        EmitterShape::Cone { height, radius } => {
            let init_pos = SetPositionCone3dModifier {
                base_radius: writer.lit(0.).expr(),
                top_radius: writer.lit(*radius).expr(),
                height: writer.lit(*height).expr(),
                dimension: ShapeDimension::Volume,
            };
            Box::new(init_pos)
        },
        EmitterShape::Box { size } => {
            let init_pos = SetPositionCircleModifier {
                center: writer.lit(Vec3::ZERO).expr(),
                radius: writer.lit(size.length() * 0.5).expr(),
                axis: writer.lit(Vec3::Y).expr(),
                dimension: ShapeDimension::Surface,
            };
            Box::new(init_pos)
        },
    }
}

pub fn setup_particle_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    _error_manager: ResMut<ErrorManager>,
) {
    let mut writer = ExprWriter::new();
    
    // Create gradient from our unified color system
    let gradient = {
        let mut gradient = Gradient::new();
        for (pos, color) in particle_gradient() {
            gradient.add_key(pos, color.as_vec4());
        }
        gradient
    };

    let effect = EffectAsset::new(32768, Spawner::rate(30.0.into()), writer.clone().finish())
        .with_name("fallback")
        .init(SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(1.0).expr()))
        .init(SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(0.2).expr(),
            dimension: ShapeDimension::Surface,
        })
        .render(ColorOverLifetimeModifier { gradient });

    commands.insert_resource(MagneticEffects {
        north: effects.add(effect.clone()),
        south: effects.add(effect.clone()),
        fallback: effects.add(effect),
    });
}

pub fn update_particles(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut ParticleEmitter, &Transform)>,
    time: Res<Time>,
    mut error_manager: ResMut<ErrorManager>,
) {
    for (entity, mut emitter, _transform) in particles.iter_mut() {
        if !emitter.is_active() {
            continue;
        }

        if let Err(e) = emitter.try_update(time.delta_seconds()) {
            error_manager.report_error(e);
            continue;
        }

        if emitter.is_complete() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_particle_emitter(
    commands: &mut Commands,
    magnetic_effects: &MagneticEffects,
    position: Vec3,
    config: ParticleEmitter,
    error_manager: &mut ErrorManager,
) {
    if let Err(e) = config.validate() {
        error_manager.report_with_recovery(
            e,
            "Using fallback particle configuration"
        );
        commands.spawn((
            ParticleEffectBundle {
                effect: bevy_hanabi::ParticleEffect::new(magnetic_effects.fallback.clone()),
                transform: Transform::from_translation(position),
                ..default()
            },
            ParticleEmitter::default(),
        ));
        return;
    }

    let effect_handle = match config.get_polarity() {
        true => magnetic_effects.north.clone(),
        false => magnetic_effects.south.clone(),
    };

    commands.spawn((
        ParticleEffectBundle {
            effect: bevy_hanabi::ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(position),
            ..default()
        },
        config,
    ));
}

pub fn draw_mesh_intersections(
    pointers: Query<&PointerInteraction>, 
    mut gizmos: Gizmos
) {
    use crate::resources::uni_color::presets::gizmo_colors;
    let (point_color, arrow_color) = gizmo_colors();

    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, point_color.as_bevy_color());
        gizmos.arrow(point, point + normal.normalize() * 0.5, arrow_color.as_bevy_color());
    }
} 

pub fn setup_particle_effects(
    _error_manager: ResMut<ErrorManager>,
) {
    let writer = ExprWriter::new();
    // ... rest of the function
} 

pub fn update_particle_effects(
    mut query: Query<(&mut ParticleEffect, &mut Transform)>,
    time: Res<Time>,
    mut error_manager: ResMut<ErrorManager>,
) {
    for (mut effect, mut transform) in query.iter_mut() {
        if let Err(e) = update_effect(&mut effect, &mut transform, time.delta_seconds()) {
            error_manager.report_error(e);
            continue;
        }
    }
}

#[derive(Component)]
pub struct ParticleEffect {
    pub handle: Handle<EffectAsset>,
    pub z_layer_2d: f32,
    pub lifetime: f32,
    pub scale_rate: f32,
    pub rotation_rate: f32,
}

impl ParticleEffect {
    pub fn new(effect_handle: Handle<EffectAsset>) -> Self {
        Self {
            handle: effect_handle,
            ..Default::default()
        }
    }
}

impl Default for ParticleEffect {
    fn default() -> Self {
        Self {
            handle: Handle::default(),
            z_layer_2d: 0.0,
            lifetime: 5.0,
            scale_rate: 0.1,
            rotation_rate: 0.1,
        }
    }
}

pub fn update_effect(
    effect: &mut ParticleEffect,
    transform: &mut Transform,
    delta: f32
) -> Result<bool> {
    effect.lifetime -= delta;
    if effect.lifetime <= 0.0 {
        return Err(Error::Component(ComponentError::StateError(
            "Particle effect lifetime expired".into()
        )));
    }

    let new_scale = transform.scale + Vec3::splat(effect.scale_rate * delta);
    if !new_scale.is_finite() {
        return Err(Error::Component(ComponentError::ValidationFailed(
            "Invalid scale value".into()
        )));
    }
    transform.scale = new_scale;

    let rotation = Quat::from_rotation_y(effect.rotation_rate * delta);
    if !rotation.is_finite() {
        return Err(Error::Component(ComponentError::ValidationFailed(
            "Invalid rotation value".into()
        )));
    }
    transform.rotate(rotation);

    Ok(effect.lifetime > 0.0 && effect.lifetime.is_finite())
} 