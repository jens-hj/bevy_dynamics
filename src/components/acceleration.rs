use bevy::prelude::*;

use super::{Mass, Velocity};

/// Bevy [`Component`] representing an [`Entity`]'s acceleration.
///
/// This component requires the [`Velocity`], the [`Transform`] and
/// [`crate::Damping`] components.
#[derive(Component)]
#[require(Velocity)]
pub struct Acceleration {
    /// The acceleration value in 3D space.
    pub value: Vec3,
}

impl Acceleration {
    /// Create a new [`Acceleration`] component with the given 3D value.
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }

    /// Apply a force to the [`Entity`] and update the acceleration.
    pub fn apply_force(&mut self, force: Vec3, mass: &Mass) {
        self.value += force / mass.value;
    }

    /// Get the magnitude of the acceleration.
    pub fn magnitude(&self) -> f32 {
        self.value.length()
    }
}

impl Default for Acceleration {
    /// Create a new [`Acceleration`] component with the default value of `[0,
    /// 0, 0]`.
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}
