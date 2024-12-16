use bevy::prelude::*;

#[derive(Component, Default)]
pub struct WindowBorder {
    pub color: Color,
    pub width: f32,
}

impl WindowBorder {
    pub fn new(color: Color, width: f32) -> Self {
        Self { color, width }
    }
} 