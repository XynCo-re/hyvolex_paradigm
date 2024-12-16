use bevy::{
    prelude::*,
    render::{
        mesh::{Mesh, Indices},
        render_resource::PrimitiveTopology,
        render_asset::RenderAssetUsages,
    },
};
use crate::components::node::ShapeType;

#[derive(Debug, Clone, Component)]
pub enum Shape {
    Box(Box3d),
    Sphere(Sphere3d),
    Capsule(Capsule3d),
    Cylinder(Cylinder3d),
    Cone(Cone3d),
    Torus(Torus3d),
}

#[derive(Debug, Clone, Default)]
pub struct Box3d {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Sphere3d {
    pub radius: f32,
    pub sectors: usize,
    pub stacks: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Capsule3d {
    pub radius: f32,
    pub height: f32,
    pub rings: usize,
    pub latitudes: usize,
    pub longitudes: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Cylinder3d {
    pub radius: f32,
    pub height: f32,
    pub resolution: usize,
    pub segments: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Cone3d {
    pub radius: f32,
    pub height: f32,
    pub resolution: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Torus3d {
    pub radius: f32,
    pub ring_radius: f32,
    pub rings: usize,
    pub sectors: usize,
}

impl Shape {
    pub fn from_node_shape(shape_type: ShapeType) -> Self {
        let (radius, length) = shape_type.dimensions();
        match shape_type {
            ShapeType::Alpha => Shape::Capsule(Capsule3d {
                radius,
                height: length,
                ..default()
            }),
            ShapeType::Beta => Shape::Cylinder(Cylinder3d {
                radius,
                height: length,
                ..default()
            }),
            ShapeType::Gamma => Shape::Cone(Cone3d {
                radius,
                height: length,
                ..default()
            }),
        }
    }

    pub fn create_mesh(&self) -> Mesh {
        match self {
            Shape::Box(box3d) => {
                Mesh::from(bevy::prelude::shape::Box::new(
                    box3d.width,
                    box3d.height,
                    box3d.depth,
                ))
            }
            Shape::Sphere(sphere) => {
                Mesh::from(bevy::prelude::shape::UVSphere {
                    radius: sphere.radius,
                    sectors: sphere.sectors,
                    stacks: sphere.stacks,
                })
            }
            Shape::Capsule(capsule) => {
                Mesh::from(bevy::prelude::shape::Capsule {
                    radius: capsule.radius,
                    depth: capsule.height,
                    rings: capsule.rings,
                    latitudes: capsule.latitudes,
                    longitudes: capsule.longitudes,
                    uv_profile: bevy::prelude::shape::CapsuleUvProfile::Uniform,
                })
            }
            Shape::Cylinder(cylinder) => {
                Mesh::from(bevy::prelude::shape::Cylinder {
                    radius: cylinder.radius,
                    height: cylinder.height,
                    resolution: cylinder.resolution,
                    segments: cylinder.segments,
                })
            }
            Shape::Cone(cone) => {
                Mesh::from(bevy::prelude::shape::Cylinder {
                    radius: cone.radius,
                    height: cone.height,
                    resolution: cone.resolution,
                    segments: 1,
                })
            }
            Shape::Torus(torus) => {
                Mesh::from(bevy::prelude::shape::Torus {
                    radius: torus.radius,
                    ring_radius: torus.ring_radius,
                    rings: torus.rings,
                    sectors: torus.sectors,
                })
            }
        }
    }
} 