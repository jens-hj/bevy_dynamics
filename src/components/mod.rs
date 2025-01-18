//! This module contains the components that are used by the [`DynamicsPlugin`].
//!
//! Components such as [`Velocity`], [`Acceleration`], [`Mass`] and [`Damping`],
//! but also optional components such as [`debug::Debug`],
//! [`debug::DebugColors`] and [`debug::DebugScale`] hidden behind the `debug`
//! feature flag.

mod acceleration;
mod damping;
#[cfg(feature = "debug")]
mod debug;
mod mass;
mod velocity;

pub use acceleration::*;
pub use damping::*;
#[cfg(feature = "debug")]
pub use debug::*;
pub use mass::*;
pub use velocity::*;
