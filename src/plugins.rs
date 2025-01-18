//! The `plugins` module contains the [`DynamicsPlugin`].

use bevy::prelude::*;

use crate::apply_dynamics;
#[cfg(feature = "debug")]
use crate::debug;

/// The Bevy [`Plugin`] for the [`DynamicsPlugin`].
pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        {
            app.add_systems(Update, debug);
        }

        app.add_systems(FixedUpdate, apply_dynamics);
    }
}
