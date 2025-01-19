use bevy::prelude::*;
use bevy_dynamics::{Acceleration, Damping, Mass, Velocity};
#[cfg(feature = "debug")]
use bevy_dynamics::{Debug, DebugColors, DebugScale};

use super::{
    components::{
        AccelerationMagnitudeText, AccelerationVectorText,
        VelocityMagnitudeText, VelocityVectorText,
    },
    constants::{ACCELERATION_COLOR, VELOCITY_COLOR},
    ParticleConfiguration, SpawnConfiguration,
};

/// Bevy [`Startup`] system that sets up the scene with a camera and a dynamics
/// entity with a mass of 1.0, an initial velocity of [0, 0, 0], an initial
/// acceleration of [0, 0, 0], and a damping of 0.05.
///
/// Also sets up debugging if the `debug` feature is enabled.
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawn_config: Res<SpawnConfiguration>,
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
    for particle in spawn_config.particles.iter() {
        #[cfg(not(feature = "debug"))]
        spawn_particle(
            &mut commands,
            particle.clone(),
            &mut meshes,
            &mut materials,
        );

        #[cfg(feature = "debug")]
        {
            let entity = spawn_particle(
                &mut commands,
                particle.clone(),
                &mut meshes,
                &mut materials,
            );

            commands.entity(entity).insert((
                Debug::default(),
                DebugColors {
                    velocity: VELOCITY_COLOR,
                    acceleration: ACCELERATION_COLOR,
                },
                DebugScale { scale: SCALE },
            ));
        }
    }
}

/// Utility function to spawn a particle and return the entity.
pub fn spawn_particle(
    commands: &mut Commands,
    particle: ParticleConfiguration,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(particle.radius))),
            MeshMaterial3d(
                materials.add(StandardMaterial::from_color(particle.color)),
            ),
            Transform::from_translation(particle.position),
            Mass::new(particle.mass),
            Velocity::new(particle.velocity),
            Acceleration::new(particle.acceleration),
            Damping::new(particle.damping),
            PickingBehavior::IGNORE,
            particle,
        ))
        .id()
}

/// Bevy [`Startup`] system that creates the text nodes for the velocity and
/// acceleration.
pub fn setup_text(
    mut commands: Commands,
    spawn_config: Res<SpawnConfiguration>,
) {
    // text in the top left corner
    commands
        .spawn(Node {
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            for particle in spawn_config.particles.iter() {
                // title for the particle which will be the particle's mass
                parent.spawn((
                    Text::new(format!("Mass: {}", particle.mass)),
                    TextColor(particle.color),
                ));

                // Showing the velocity as "Velocity = |[v.x, v.y, v.z]| =
                // ||v||"
                parent
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Velocity = "),
                            TextColor(VELOCITY_COLOR.into()),
                        ));

                        parent.spawn((
                            Text::new("||[0, 0, 0]||"),
                            TextColor(VELOCITY_COLOR.into()),
                            VelocityVectorText,
                            particle.clone(),
                        ));

                        parent.spawn((
                            Text::new(" = "),
                            TextColor(VELOCITY_COLOR.into()),
                        ));

                        parent.spawn((
                            Text::new("0"),
                            TextColor(VELOCITY_COLOR.into()),
                            VelocityMagnitudeText,
                            particle.clone(),
                        ));
                    });

                // Showing the acceleration as "Acceleration = |[a.x, a.y, a.z]|
                // = ||a||"
                parent
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Acceleration = "),
                            TextColor(ACCELERATION_COLOR.into()),
                        ));

                        parent.spawn((
                            Text::new("||[0, 0, 0]||"),
                            TextColor(ACCELERATION_COLOR.into()),
                            AccelerationVectorText,
                            particle.clone(),
                        ));

                        parent.spawn((
                            Text::new(" = "),
                            TextColor(ACCELERATION_COLOR.into()),
                        ));

                        parent.spawn((
                            Text::new("0"),
                            TextColor(ACCELERATION_COLOR.into()),
                            AccelerationMagnitudeText,
                            particle.clone(),
                        ));
                    });

                // Spacer
                parent.spawn(Node {
                    height: Val::Px(20.0),
                    ..default()
                });
            }
        });
}

/// Bevy [`Update`] system that updates the velocity text.
pub fn update_velocity_text(
    query: Query<(&Velocity, &ParticleConfiguration)>,
    mut query_vector_text: Query<
        (&mut Text, &ParticleConfiguration),
        (With<VelocityVectorText>, Without<VelocityMagnitudeText>),
    >,
    mut query_magnitude_text: Query<
        (&mut Text, &ParticleConfiguration),
        (With<VelocityMagnitudeText>, Without<VelocityVectorText>),
    >,
) {
    for (velocity, config) in query.iter() {
        for (mut vector_text, other_config) in query_vector_text.iter_mut() {
            if other_config.name == config.name {
                vector_text.0 = format!(
                    "||[{:.2}, {:.2}, {:.2}]||",
                    velocity.value.x, velocity.value.y, velocity.value.z
                );
            }
        }

        for (mut magnitude_text, other_config) in
            query_magnitude_text.iter_mut()
        {
            if other_config.name == config.name {
                magnitude_text.0 = format!("{:.2}", velocity.speed());
            }
        }
    }
}

/// Bevy [`Update`] system that updates the acceleration text.
pub fn update_acceleration_text(
    query: Query<(&Acceleration, &ParticleConfiguration)>,
    mut query_vector_text: Query<
        (&mut Text, &ParticleConfiguration),
        (
            With<AccelerationVectorText>,
            Without<AccelerationMagnitudeText>,
        ),
    >,
    mut query_magnitude_text: Query<
        (&mut Text, &ParticleConfiguration),
        (
            With<AccelerationMagnitudeText>,
            Without<AccelerationVectorText>,
        ),
    >,
) {
    for (acceleration, config) in query.iter() {
        for (mut vector_text, other_config) in query_vector_text.iter_mut() {
            if other_config.name == config.name {
                vector_text.0 = format!(
                    "||[{:.2}, {:.2}, {:.2}]||",
                    acceleration.value.x,
                    acceleration.value.y,
                    acceleration.value.z
                );
            }
        }

        for (mut magnitude_text, other_config) in
            query_magnitude_text.iter_mut()
        {
            if other_config.name == config.name {
                magnitude_text.0 = format!("{:.2}", acceleration.magnitude());
            }
        }
    }
}
