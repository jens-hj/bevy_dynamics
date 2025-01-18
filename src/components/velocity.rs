use bevy::prelude::*;

use super::{Acceleration, Damping, Mass};

/// The speed of light in meters per second.
const SPEED_OF_LIGHT: f32 = 299_792_458.0;

/// Bevy [`Component`] representing an [`Entity`]'s velocity.
/// This component requires the [`Transform`] and [`Damping`] components.
#[derive(Component)]
#[require(Transform, Damping)]
pub struct Velocity {
    /// The velocity value in 3D space.
    pub value: Vec3,
}

impl Velocity {
    /// Create a new [`Velocity`] component with the given 3D value.
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }

    /// Apply an acceleration to the velocity.
    ///
    /// Clamps the velocity to the speed of light [`SPEED_OF_LIGHT`].
    pub fn apply_acceleration(
        &mut self,
        acceleration: &Acceleration,
        delta_time_secs: f32,
    ) {
        let vel = self.value + acceleration.value * delta_time_secs;
        if vel.length() > SPEED_OF_LIGHT {
            self.value = vel.normalize() * SPEED_OF_LIGHT;
        } else {
            self.value = vel;
        }
    }

    /// Apply an impulse to the velocity.
    pub fn apply_impulse(
        &mut self,
        impulse: Vec3,
        mass: &Mass,
        delta_time_secs: f32,
    ) {
        self.value += impulse / mass.value * delta_time_secs;
    }

    /// Apply damping to the velocity to mimic friction.
    pub fn apply_damping(&mut self, damping: &Damping, delta_time_secs: f32) {
        self.value *= 1.0 - damping.value * delta_time_secs;

        // like air resistance damping should be proportional to the square of
        // the speed
        // self.value *=
        //     1.0 - damping.value * self.value.length_squared() *
        // delta_time_secs;
    }

    /// Get the current speed.
    pub fn speed(&self) -> f32 {
        self.value.length()
    }
}

impl Default for Velocity {
    /// Create a new [`Velocity`] component with the default value of `[0, 0,
    /// 0]`.
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}
