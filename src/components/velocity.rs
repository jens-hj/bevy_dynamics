use bevy::prelude::*;

use super::Acceleration;

const DAMPING_FACTOR: f32 = 0.95;
const SPEED_OF_LIGHT: f32 = 299792458.0;

#[derive(Component)]
#[require(Transform)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }

    pub fn apply_acceleration(&mut self, acceleration: &Acceleration, delta_time_secs: f32) {
        // Damping factor to reduce the velocity and induce stability
        self.value *= DAMPING_FACTOR;

        let vel = self.value + acceleration.value * delta_time_secs;
        if vel.length() > SPEED_OF_LIGHT {
            self.value = vel.normalize() * SPEED_OF_LIGHT;
        } else {
            self.value = vel;
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}
