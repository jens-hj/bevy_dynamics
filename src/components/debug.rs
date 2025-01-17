#[cfg(feature = "debug")]
use bevy::prelude::*;

#[cfg(feature = "debug")]
#[derive(Component)]
pub struct Debug {
    pub enabled: bool,
}

impl Default for Debug {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[cfg(feature = "debug")]
#[derive(Component)]
pub struct DebugColors {
    pub velocity: Color,
    pub acceleration: Color,
}

impl Default for DebugColors {
    fn default() -> Self {
        Self {
            velocity: Color::srgb_u8(255, 0, 0),
            acceleration: Color::srgb_u8(0, 255, 0),
        }
    }
}

#[cfg(feature = "debug")]
#[derive(Component)]
pub struct DebugScale {
    pub scale: f32,
}

impl Default for DebugScale {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}
