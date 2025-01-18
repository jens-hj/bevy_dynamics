use bevy::{
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::*,
};
use dynamics::{Acceleration, Damping, DynamicsPlugin, Velocity};
#[cfg(feature = "debug")]
use dynamics::{Debug, DebugColors, DebugScale};

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

    // Setup the scene and text
    app.add_systems(Startup, (setup_scene, setup_text));

    // Update the text when the velocity or acceleration changes
    app.add_systems(Update, (update_velocity_text, update_acceleration_text));

    // Update the acceleration with a sine wave in x and z over time
    app.add_systems(FixedUpdate, update_acceleration);

    app.run();
}

/// Bevy [`Startup`] system that sets up the scene with a camera and a dynamics
/// entity with a mass of 1.0, an initial velocity of [0, 0, 0], an initial
/// acceleration of [1, 0, 1], and a damping of 0.05.
///
/// Also sets up debugging if the `debug` feature is enabled.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    #[cfg(feature = "debug")]
    const SCALE: f32 = 1.0;

    // Add camera and light
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));
    commands.spawn(DirectionalLight::default());

    // Add moving entity with debug visualization
    let entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(
                bevy_catppuccin::Flavor::MOCHA.blue,
            ))),
            Transform::from_xyz(-0.5, 0.0, 0.0),
            Velocity::new(Vec3::new(0.0, 0.0, 0.0)),
            Acceleration::new(Vec3::new(1.0, 0.0, 1.0)),
            Damping::new(0.05),
        ))
        .id();

    #[cfg(feature = "debug")]
    commands.entity(entity).insert((
        Debug::default(),
        DebugColors {
            velocity: VELOCITY_COLOR,
            acceleration: ACCELERATION_COLOR,
        },
        DebugScale { scale: SCALE },
    ));
}

/// Bevy [`FixedUpdate`] system that updates the acceleration with a sine wave
/// in x and z over time
fn update_acceleration(
    mut query: Query<&mut Acceleration>,
    time: Res<Time<Fixed>>,
) {
    const AMPLITUDE: f32 = 5.0;

    for mut acceleration in query.iter_mut() {
        let x = AMPLITUDE * time.elapsed_secs().sin();
        let z = AMPLITUDE * time.elapsed_secs().cos();
        acceleration.value = Vec3::new(x, 0.0, z);
    }
}
