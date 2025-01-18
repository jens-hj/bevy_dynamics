//! The `systems` module contains the [`apply_acceleration`] and
//! [`apply_velocity`] systems.
//!
//! Along with the [`debug_acceleration`] and [`debug_velocity`] systems, which
//! are only available when the `debug` feature is enabled.

use bevy::prelude::*;

use crate::{Acceleration, Damping, Velocity};
#[cfg(feature = "debug")]
use crate::{Debug, DebugColors, DebugScale};

/// Applies [`Acceleration`], [`Damping`], and [`Velocity`] changes in a single
/// pass.
///
/// This system is run on the [`FixedUpdate`] schedule.
pub fn apply_dynamics(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        Option<&Acceleration>,
        Option<&Damping>,
    )>,
    time: Res<Time<Fixed>>,
) {
    for (mut transform, mut velocity, acceleration, damping) in query.iter_mut()
    {
        // Apply acceleration if component exists
        if let Some(acceleration) = acceleration {
            velocity.apply_acceleration(&acceleration, time.delta_secs());
        }

        // Apply damping if component exists
        if let Some(damping) = damping {
            velocity.apply_damping(damping, time.delta_secs());
        }

        // Apply velocity to transform
        transform.translation += velocity.value * time.delta_secs();
    }
}

/// Debugs the [`Acceleration`] and [`Velocity`] components by drawing arrows
/// in the scene with the Bevy [`Gizmos`].
#[cfg(feature = "debug")]
pub fn debug(
    mut gizmos: Gizmos,
    query: Query<(
        &Transform,
        &Velocity,
        &Acceleration,
        &Debug,
        &DebugColors,
        &DebugScale,
    )>,
) {
    for (transform, velocity, acceleration, debug, colors, scale) in
        query.iter()
    {
        if debug.velocity {
            gizmos.arrow(
                transform.translation,
                transform.translation + velocity.value * scale.scale,
                colors.velocity,
            );
        }

        if debug.acceleration {
            gizmos.arrow(
                transform.translation,
                transform.translation + acceleration.value * scale.scale,
                colors.acceleration,
            );
        }
    }
}
