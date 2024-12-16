use bevy::{
    prelude::*,
    render::color::LinearRgba,
};

/// A unified color type that handles conversions between different color formats
/// needed across the application.
#[derive(Debug, Clone, Copy)]
pub struct UniColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl UniColor {
    /// Create a new color from sRGB values
    pub fn srgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a new color from sRGBA values
    pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new color from linear RGB values
    pub fn linear_rgb(r: f32, g: f32, b: f32) -> Self {
        let color = Color::rgb_linear(r, g, b);
        Self::from(color)
    }

    /// Create a new color from linear RGBA values
    pub fn linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        let color = Color::rgba_linear(r, g, b, a);
        Self::from(color)
    }

    /// Get the color as a Bevy Color (sRGB)
    pub fn as_bevy_color(&self) -> Color {
        Color::srgba(self.r, self.g, self.b, self.a)
    }

    /// Get the color as a linear RGBA color for materials
    pub fn as_linear_rgba(&self) -> LinearRgba {
        let color = self.as_bevy_color().as_linear_rgba_f32();
        LinearRgba::new(color[0], color[1], color[2], color[3])
    }

    /// Get the color as a Vec4 for particle systems
    pub fn as_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// Get the color with a modified alpha value
    pub fn with_alpha(self, alpha: f32) -> Self {
        Self {
            a: alpha.clamp(0.0, 1.0),
            ..self
        }
    }

    /// Validates that all color components are finite and within valid ranges
    pub fn is_valid(&self) -> bool {
        self.r >= 0.0 && self.r <= 1.0 &&
        self.g >= 0.0 && self.g <= 1.0 &&
        self.b >= 0.0 && self.b <= 1.0 &&
        self.a >= 0.0 && self.a <= 1.0
    }

    /// Predefined colors
    pub fn red() -> Self {
        Self::srgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::srgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::srgb(0.0, 0.0, 1.0)
    }

    pub fn white() -> Self {
        Self::srgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::srgb(0.0, 0.0, 0.0)
    }

    pub fn transparent() -> Self {
        Self::srgba(0.0, 0.0, 0.0, 0.0)
    }
}

// Conversion from Bevy Color
impl From<Color> for UniColor {
    fn from(color: Color) -> Self {
        let [r, g, b, a] = color.as_rgba_f32();
        Self { r, g, b, a }
    }
}

// Conversion to Bevy Color
impl From<UniColor> for Color {
    fn from(color: UniColor) -> Self {
        Color::rgba(color.r, color.g, color.b, color.a)
    }
}

// Conversion to Vec4 for particle systems
impl From<UniColor> for Vec4 {
    fn from(color: UniColor) -> Self {
        Vec4::new(color.r, color.g, color.b, color.a)
    }
}

/// Material color configuration with proper color handling
#[derive(Resource)]
pub struct MaterialColors {
    pub node: UniColor,
    pub connection: UniColor,
    pub highlight: UniColor,
    pub background: UniColor,
    pub emissive: UniColor,
}

impl Default for MaterialColors {
    fn default() -> Self {
        Self {
            node: UniColor::srgba(0.9, 0.9, 0.9, 0.9),
            connection: UniColor::srgb(0.8, 0.2, 0.2),
            highlight: UniColor::srgb(0.2, 0.2, 0.8),
            background: UniColor::srgb(0.1, 0.1, 0.1),
            emissive: UniColor::srgb(0.2, 0.2, 1.0),
        }
    }
}

/// Helper functions for common color operations
pub mod presets {
    use super::*;

    pub fn debug_colors() -> Vec<UniColor> {
        vec![
            UniColor::red(),
            UniColor::green(),
            UniColor::blue(),
            UniColor::srgb(1.0, 1.0, 0.0), // Yellow
            UniColor::srgb(1.0, 0.0, 1.0), // Magenta
            UniColor::srgb(0.0, 1.0, 1.0), // Cyan
        ]
    }

    pub fn particle_gradient() -> Vec<(f32, UniColor)> {
        vec![
            (0.0, UniColor::srgba(1.0, 0.0, 0.0, 1.0)),
            (0.5, UniColor::srgba(1.0, 1.0, 0.0, 0.8)),
            (1.0, UniColor::srgba(0.0, 0.0, 1.0, 0.0)),
        ]
    }

    pub fn gizmo_colors() -> (UniColor, UniColor) {
        (
            UniColor::srgb(1.0, 0.3, 0.3), // Point color
            UniColor::srgb(0.3, 1.0, 0.3), // Arrow color
        )
    }
} 