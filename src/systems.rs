use bevy::prelude::*;

use crate::{Acceleration, Velocity};
#[cfg(feature = "debug")]
use crate::{Debug, DebugColors, DebugScale};

pub fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.apply_acceleration(acceleration, time.delta_secs());
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}

#[cfg(feature = "debug")]
pub fn debug_acceleration(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Acceleration, &Debug, &DebugColors, &DebugScale)>,
) {
    for (transform, acceleration, debug, colors, scale) in query.iter() {
        if !debug.enabled {
            continue;
        }

        gizmos.line(
            transform.translation,
            transform.translation + acceleration.value * scale.scale,
            colors.acceleration,
        );
    }
}

#[cfg(feature = "debug")]
pub fn debug_velocity(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Velocity, &Debug, &DebugColors, &DebugScale)>,
) {
    for (transform, velocity, debug, colors, scale) in query.iter() {
        if !debug.enabled {
            continue;
        }

        gizmos.line(
            transform.translation,
            transform.translation + velocity.value * scale.scale,
            colors.velocity,
        );
    }
}
