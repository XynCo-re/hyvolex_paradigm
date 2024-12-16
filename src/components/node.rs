// src/components/node.rs
use bevy::prelude::*;
use crate::components::magnetic_field::MagneticField;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Node {
    pub shape_type: ShapeType,
    pub rotation: f32,
    pub magnetic_field: MagneticField,
    pub temporal_phase: f32,
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq)]
pub enum ShapeType {
    Alpha,
    Beta,
    Gamma,
}

impl ShapeType {
    pub fn dimensions(&self) -> (f32, f32) { // (radius, length)
        match self {
            ShapeType::Alpha => (0.2, 0.6),
            ShapeType::Beta => (0.3, 0.8),
            ShapeType::Gamma => (0.4, 1.0),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            shape_type: ShapeType::Alpha,
            rotation: 0.0,
            magnetic_field: MagneticField::default(),
            temporal_phase: 0.0,
        }
    }
}