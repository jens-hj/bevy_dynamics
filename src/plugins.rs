use bevy::prelude::*;

use crate::{apply_acceleration, apply_velocity};

#[cfg(feature = "debug")]
use crate::{debug_acceleration, debug_velocity};

pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut App) {
        let systems = (apply_acceleration, apply_velocity);

        #[cfg(feature = "debug")]
        {
            app.add_systems(Update, (systems, debug_acceleration, debug_velocity));
        }

        #[cfg(not(feature = "debug"))]
        {
            app.add_systems(Update, systems);
        }
    }
}
