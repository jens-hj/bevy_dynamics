use bevy::prelude::*;

use super::Acceleration;

/// Bevy [`Component`] representing an [`Entity`]'s mass.
/// This component requires the [`Acceleration`] component, thus also requiring
/// the [`crate::components::Velocity`], [`crate::components::Damping`] and
/// [`Transform`] components.
#[derive(Component)]
#[require(Acceleration)]
pub struct Mass {
    /// Mass in kilograms
    pub value: f32,
}

impl Default for Mass {
    /// Create a new [`Mass`] component with the default value of `1.0`.
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Mass {
    /// Create a new [`Mass`] component with the given `f32` value.
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
