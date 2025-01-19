use bevy::prelude::*;
use dynamics::{Acceleration, DynamicsPlugin};

mod common;
use common::*;

fn main() {
    let mut app = App::new();

    // Determine the fixed update rate
    app.insert_resource(Time::<Fixed>::from_hz(100.0));
    app.add_plugins((DefaultPlugins, DynamicsPlugin));

    // Setup the visuals
    app.insert_resource(ClearColor(bevy_catppuccin::Flavor::MOCHA.base));
    app.insert_resource(AmbientLight {
        brightness: 200.0,
        ..default()
    });

    // Setup the particles
    let particles = vec![ParticleConfiguration {
        name: "Particle 1".into(),
        mass: 1.0,
        radius: 0.1,
        color: bevy_catppuccin::Flavor::MOCHA.blue,
        position: Vec3::new(-2.0, 0.0, 0.0),
        velocity: Vec3::ZERO,
        acceleration: Vec3::ZERO,
        damping: 0.5,
    }];
    app.insert_resource(SpawnConfiguration::new(particles));

    // Setup the scene and text
    app.add_systems(Startup, (setup_scene, setup_text));

    // Update the text when the velocity or acceleration changes
    app.add_systems(Update, (update_velocity_text, update_acceleration_text));

    // Update the acceleration with a sine wave in x and z over time
    app.add_systems(FixedUpdate, update_acceleration);

    app.run();
}

/// Bevy [`FixedUpdate`] system that updates the acceleration with a sine wave
/// in x and z over time
fn update_acceleration(
    mut query: Query<&mut Acceleration>,
    time: Res<Time<Fixed>>,
) {
    const AMPLITUDE: f32 = 1.0;

    for mut acceleration in query.iter_mut() {
        let x = AMPLITUDE * time.elapsed_secs().sin();
        let z = AMPLITUDE * time.elapsed_secs().cos();
        acceleration.value = Vec3::new(x, 0.0, z);
    }
}
