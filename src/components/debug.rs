#[cfg(feature = "debug")]
use bevy::prelude::*;

/// Bevy [`Component`] determining if the debug vectors should be drawn.
/// Flip this at runtime to toggle the debug vectors on and off.
#[cfg(feature = "debug")]
#[derive(Component)]
#[require(DebugColors, DebugScale)]
pub struct Debug {
    /// Whether the velocity debug vector should be drawn.
    pub velocity: bool,
    /// Whether the acceleration debug vector should be drawn.
    pub acceleration: bool,
}

impl Default for Debug {
    /// Create a new [`Debug`] component with the default value of `true`.
    fn default() -> Self {
        Self {
            velocity: true,
            acceleration: true,
        }
    }
}

/// Bevy [`Component`] determining the colors of the [`Velocity`] and
/// [`Acceleration`] debug vectors.
#[cfg(feature = "debug")]
#[derive(Component)]
pub struct DebugColors {
    /// The color of the velocity vector.
    pub velocity: Color,
    /// The color of the acceleration vector.
    pub acceleration: Color,
}

impl Default for DebugColors {
    /// Create a new [`DebugColors`] component with the default values of `[255,
    /// 0, 0]` for [`Velocity`] and `[0, 255, 0]` for [`Acceleration`].
    fn default() -> Self {
        Self {
            velocity: Color::srgb_u8(255, 0, 0),
            acceleration: Color::srgb_u8(0, 255, 0),
        }
    }
}

/// Bevy [`Component`] determining the scale of the [`Velocity`] and
/// [`Acceleration`] debug vectors.
#[cfg(feature = "debug")]
#[derive(Component)]
pub struct DebugScale {
    /// The scale of the debug vectors.
    /// Increase or decrease to make the arrows' size appropriate for your
    /// application.
    pub scale: f32,
}

impl Default for DebugScale {
    /// Create a new [`DebugScale`] component with the default value of `1.0`.
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}
