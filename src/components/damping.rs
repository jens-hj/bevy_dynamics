use bevy::prelude::*;

/// Damping to simulate friction
#[derive(Component)]
pub struct Damping {
    /// Damping coefficient
    pub value: f32,
}

impl Default for Damping {
    /// Create a new [`Damping`] component with the default value of `0.0`.
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl Damping {
    /// Create a new [`Damping`] component with an f32 value.
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
