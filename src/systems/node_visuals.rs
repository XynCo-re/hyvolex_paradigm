use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::{
    components::{MagneticField, Polarity, MeshMaterial3d},
    resources::{Materials, uni_color::UniColor},
    err::{Result, SystemError},
};
use bevy::math::Vec4;

#[derive(Resource)]
pub struct NodeEffects {
    pub active: Handle<EffectAsset>,
    pub highlight: Handle<EffectAsset>,
    pub pulse: Handle<EffectAsset>,
}

pub fn setup_node_effects(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) -> Result<()> {
    let writer = ExprWriter::new();

    // Base properties
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.5).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Active node effect (constant gentle particle emission)
    let mut active_gradient = Gradient::new();
    active_gradient.add_key(0.0, Vec4::new(0.5, 0.8, 1.0, 0.8));
    active_gradient.add_key(0.5, Vec4::new(0.4, 0.6, 0.9, 0.4));
    active_gradient.add_key(1.0, Vec4::new(0.3, 0.4, 0.8, 0.0));

    let active_effect = create_node_effect(
        &mut effects,
        writer.clone(),
        active_gradient,
        init_age.clone(),
        init_lifetime.clone(),
        30.0,
        0.3,
    )?;

    // Highlight effect (more intense, faster particles)
    let mut highlight_gradient = Gradient::new();
    highlight_gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.3, 1.0));
    highlight_gradient.add_key(0.3, Vec4::new(0.9, 0.8, 0.2, 0.8));
    highlight_gradient.add_key(0.7, Vec4::new(0.8, 0.7, 0.1, 0.4));
    highlight_gradient.add_key(1.0, Vec4::new(0.7, 0.6, 0.0, 0.0));

    let highlight_effect = create_node_effect(
        &mut effects,
        writer.clone(),
        highlight_gradient,
        init_age.clone(),
        init_lifetime.clone(),
        50.0,
        0.5,
    )?;

    // Pulse effect (bursts of particles)
    let mut pulse_gradient = Gradient::new();
    pulse_gradient.add_key(0.0, Vec4::new(0.2, 0.5, 1.0, 1.0));
    pulse_gradient.add_key(0.2, Vec4::new(0.3, 0.6, 1.0, 0.9));
    pulse_gradient.add_key(0.8, Vec4::new(0.4, 0.7, 1.0, 0.3));
    pulse_gradient.add_key(1.0, Vec4::new(0.5, 0.8, 1.0, 0.0));

    let pulse_effect = create_node_effect(
        &mut effects,
        writer,
        pulse_gradient,
        init_age,
        init_lifetime,
        100.0,
        0.8,
    )?;

    commands.insert_resource(NodeEffects {
        active: effects.add(active_effect),
        highlight: effects.add(highlight_effect),
        pulse: effects.add(pulse_effect),
    });

    Ok(())
}

fn create_node_effect(
    _effects: &mut Assets<EffectAsset>,
    writer: ExprWriter,
    gradient: Gradient<Vec4>,
    init_age: SetAttributeModifier,
    init_lifetime: SetAttributeModifier,
    spawn_rate: f32,
    radius: f32,
) -> Result<EffectAsset> {
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(radius).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(1.0).expr(),
    };

    let size_gradient = Gradient::constant(Vec3::splat(0.1));

    Ok(EffectAsset::new(1024, Spawner::rate(spawn_rate.into()), writer.finish())
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
        .render(ColorOverLifetimeModifier { gradient }))
}

pub fn update_node_visuals(
    time: Res<Time>,
    animation_state: Res<crate::AnimationState>,
    materials: Res<MaterialHandles>,
    node_effects: Option<Res<NodeEffects>>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &MagneticField, &mut MeshMaterial3d)>,
) {
    let time_factor = time.elapsed_secs() * animation_state.speed;

    for (_entity, transform, field, mut material) in query.iter_mut() {
        // Calculate pulsing effect based on field strength and polarity
        let base_pulse = (time_factor * 2.0 + field.strength).sin() * 0.5 + 0.5;
        let field_pulse = (time_factor * 4.0 + field.orientation).cos() * 0.5 + 0.5;
        let combined_intensity = (base_pulse + field_pulse) * 0.5;
        
        let field_intensity = (field.strength / 10.0).min(1.0); // Normalize field strength
        
        // Update material
        material.0 = match field.polarity {
            Polarity::North => {
                if field_intensity > 0.7 && combined_intensity > 0.7 {
                    materials.highlight_material.clone()
                } else if field_intensity > 0.3 || combined_intensity > 0.8 {
                    materials.connection_material.clone()
                } else {
                    materials.node_material.clone()
                }
            },
            Polarity::South => {
                if field_intensity > 0.7 && combined_intensity > 0.7 {
                    materials.connection_material.clone()
                } else if field_intensity > 0.4 || combined_intensity > 0.9 {
                    materials.highlight_material.clone()
                } else {
                    materials.node_material.clone()
                }
            }
        };

        // Spawn particle effects based on node state
        if let Some(ref effects) = node_effects {
            // Always spawn base active effect
            commands.spawn(ParticleEffectBundle {
                effect: ParticleEffect::new(effects.active.clone()),
                transform: Transform::from_translation(transform.translation),
                ..default()
            });

            // Spawn highlight effect when node is highly active
            if field_intensity > 0.7 {
                commands.spawn(ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.highlight.clone()),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                });
            }

            // Spawn pulse effect on significant state changes
            if combined_intensity > 0.9 {
                commands.spawn(ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.pulse.clone()),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                });
            }
        }
    }
} 