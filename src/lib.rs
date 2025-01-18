/*!
Bevy plugin for simulating physics.

This plugin provides a set of components and systems for simulating physics.

Example usage:

```rust
use bevy::{
    color::palettes,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::*,
};
use bevy_dynamics::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DynamicsPlugin))
        .add_systems(Startup, (spawn_particle))
        .run();
}

fn spawn_particle(mut commands: Commands) {
    commands.spawn((
         Mesh3d(meshes.add(Sphere::new(0.2))),
         MeshMaterial3d(materials.add(StandardMaterial::from_color(
             palettes::basic::BLUE,
         ))),
         // [`Acceleration`] will instantiate default implementations for
         // [`Velocity`], [`Damping`], [`Mass`], and [`Transform`] [`Component`]s.
         Acceleration::new(Vec3::new(1.0, 0.0, 1.0)),
         // Add the the [`Debug`] component to enable debugging, which will draw
         // arrows in the scene to visualize the [`Acceleration`] and [`Velocity`]
         // components. This will instantiate default implementations for
         // [`DebugColors`] and [`DebugScale`] [`Component`]s.
         // Remember to enable the `debug` feature in your `Cargo.toml` file.
         Debug::default(),
    ));
}
```
*/

mod components;
mod plugins;
mod systems;

pub use components::*;
pub use plugins::*;
pub use systems::*;
