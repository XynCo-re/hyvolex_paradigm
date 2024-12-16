use bevy::{
    prelude::*,
    pbr::StandardMaterial,
};
use bevy_hanabi::prelude::*;
use crate::{
    components::{MagneticField, Polarity},
    err::{Result, SystemError},
};
use bevy::math::Vec4;

#[derive(Component)]
pub struct NodeMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct MagneticEffects {
    pub north: Handle<EffectAsset>,
    pub south: Handle<EffectAsset>,
    pub interaction: Handle<EffectAsset>,
}

pub struct MagneticProperties {
    attraction_accel: f32,
    max_attraction_speed: f32,
    sticky_factor: f32,
    shell_half_thickness: f32,
}

pub fn setup_magnetic_effects(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) -> Result<()> {
    // Create base gradients
    let mut north_gradient = Gradient::new();
    north_gradient.add_key(0.0, Vec4::new(0.0, 0.5, 1.0, 1.0));
    north_gradient.add_key(0.5, Vec4::new(0.2, 0.6, 1.0, 0.8));
    north_gradient.add_key(1.0, Vec4::new(0.4, 0.7, 1.0, 0.0));

    let mut south_gradient = Gradient::new();
    south_gradient.add_key(0.0, Vec4::new(1.0, 0.2, 0.0, 1.0));
    south_gradient.add_key(0.5, Vec4::new(1.0, 0.4, 0.2, 0.8));
    south_gradient.add_key(1.0, Vec4::new(1.0, 0.6, 0.4, 0.0));

    let mut interaction_gradient = Gradient::new();
    interaction_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    interaction_gradient.add_key(0.3, Vec4::new(0.8, 0.8, 1.0, 0.8));
    interaction_gradient.add_key(0.7, Vec4::new(0.6, 0.6, 1.0, 0.4));
    interaction_gradient.add_key(1.0, Vec4::new(0.4, 0.4, 1.0, 0.0));

    // Create the effects
    let writer = ExprWriter::new();

    // Base properties for field effects
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(2.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Force field properties
    let attraction_accel = 20.0;
    let max_attraction_speed = 5.0;
    let sticky_factor = 2.0;
    let shell_half_thickness = 0.1;

    // North pole effect
    let north_effect = create_magnetic_pole_effect(
        &mut effects,
        writer.clone(),
        north_gradient,
        init_age.clone(),
        init_lifetime.clone(),
        attraction_accel,
        max_attraction_speed,
        sticky_factor,
        shell_half_thickness,
        true,
    )?;

    // South pole effect
    let south_effect = create_magnetic_pole_effect(
        &mut effects,
        writer.clone(),
        south_gradient,
        init_age.clone(),
        init_lifetime.clone(),
        attraction_accel,
        max_attraction_speed,
        sticky_factor,
        shell_half_thickness,
        false,
    )?;

    // Interaction effect
    let interaction_effect = create_interaction_effect(
        &mut effects,
        writer,
        interaction_gradient,
        init_age,
        init_lifetime,
    )?;

    commands.insert_resource(MagneticEffects {
        north: effects.add(north_effect),
        south: effects.add(south_effect),
        interaction: effects.add(interaction_effect),
    });

    Ok(())
}

fn create_magnetic_pole_effect(
    _effects: &mut Assets<EffectAsset>,
    writer: ExprWriter,
    gradient: Gradient<Vec4>,
    init_age: SetAttributeModifier,
    init_lifetime: SetAttributeModifier,
    attraction_accel: f32,
    max_attraction_speed: f32,
    sticky_factor: f32,
    shell_half_thickness: f32,
    is_north: bool,
) -> Result<EffectAsset> {
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.5).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(if is_north { 2.0 } else { -2.0 }).expr(),
    };

    // Force field effect
    let force_field = ConformToSphereModifier {
        origin: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.0).expr(),
        influence_dist: writer.lit(5.0).expr(),
        attraction_accel: writer.lit(attraction_accel).expr(),
        max_attraction_speed: writer.lit(max_attraction_speed).expr(),
        sticky_factor: Some(writer.lit(sticky_factor).expr()),
        shell_half_thickness: Some(writer.lit(shell_half_thickness).expr()),
    };

    Ok(EffectAsset::new(4096, Spawner::rate(100.0.into()), writer.finish())
        .with_name(if is_north { "north_pole" } else { "south_pole" })
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(force_field)
        .render(ColorOverLifetimeModifier { gradient }))
}

fn create_interaction_effect(
    _effects: &mut Assets<EffectAsset>,
    writer: ExprWriter,
    gradient: Gradient<Vec4>,
    init_age: SetAttributeModifier,
    init_lifetime: SetAttributeModifier,
) -> Result<EffectAsset> {
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.2).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityTangentModifier {
        origin: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        speed: writer.lit(3.0).expr(),
    };

    let radial_accel = RadialAccelModifier::new(
        writer.lit(Vec3::ZERO).expr(),
        writer.lit(-2.0).expr(),
    );

    Ok(EffectAsset::new(2048, Spawner::rate(50.0.into()), writer.finish())
        .with_name("magnetic_interaction")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(radial_accel)
        .render(ColorOverLifetimeModifier { gradient }))
}

pub fn update_magnetic_fields(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut MagneticField)>,
    magnetic_effects: Option<Res<MagneticEffects>>,
    _commands: Commands,
) {
    let dt = time.delta_secs();

    // First, collect all field data we need
    let field_data: Vec<(Entity, Vec3, MagneticField)> = query
        .iter()
        .map(|(entity, transform, field)| {
            (entity, transform.translation, *field)
        })
        .collect();

    // Then process each field using the collected data
    for (entity, mut transform, mut field) in query.iter_mut() {
        // Update field orientation based on strength and interaction
        let orientation_change = field.strength * dt;
        field.orientation += orientation_change;
        if field.orientation > std::f32::consts::TAU {
            field.orientation -= std::f32::consts::TAU;
        }

        // Apply magnetic forces
        if let Some(_effects) = magnetic_effects.as_ref() {
            let mut total_force = Vec3::ZERO;
            let mut orientation_influence = 0.0;

            // Calculate forces from other magnetic fields using the collected data
            for &(other_entity, other_pos, other_field) in field_data.iter() {
                if entity == other_entity {
                    continue;
                }

                let direction = other_pos - transform.translation;
                let distance = direction.length();

                // Skip if too far
                if distance > field.interaction_radius + other_field.interaction_radius {
                    continue;
                }

                // Calculate base force
                let force_magnitude = match field.calculate_base_interaction(&other_field) {
                    Ok(mag) => mag,
                    Err(_) => continue,
                };

                // Apply distance falloff
                let force = direction.normalize() * force_magnitude / (distance * distance + 1.0);
                total_force += force;

                // Calculate orientation influence based on polarity interaction
                let field_direction = Vec3::new(
                    field.orientation.cos(),
                    0.0,
                    field.orientation.sin(),
                );
                let alignment = field_direction.dot(direction.normalize());
                orientation_influence += alignment * force_magnitude * 0.1;
            }

            // Apply accumulated forces with strength-based mobility
            let movement = total_force * dt * field.strength;
            transform.translation += movement;

            // Apply orientation influence with damping
            field.orientation += orientation_influence * dt;
            field.orientation *= 0.95; // Damping
        }
    }
}

/// Calculate the final interaction strength between two magnetic fields considering distance
fn calculate_interaction_strength(
    strength_a: &f32,
    strength_b: &f32,
    polarity_a: &Polarity,
    polarity_b: &Polarity,
    distance: f32,
    radius: &f32,
) -> f32 {
    let base_strength = strength_a * strength_b * (1.0 - distance / radius);
    match (polarity_a, polarity_b) {
        (Polarity::North, Polarity::South) | (Polarity::South, Polarity::North) => base_strength,
        _ => -base_strength,
    }
} 