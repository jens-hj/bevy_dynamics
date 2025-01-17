use bevy::{
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::Sphere,
    prelude::*,
};
use dynamics::{Acceleration, DynamicsPlugin, Velocity};
#[cfg(feature = "debug")]
use dynamics::{Debug, DebugColors, DebugScale};

#[derive(Component)]
struct VelocityText;

#[derive(Component)]
struct AccelerationText;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, DynamicsPlugin));

    app.insert_resource(ClearColor(bevy_catppuccin::Flavor::MOCHA.base));

    // Add a simple entity with velocity, acceleration, and debug
    app.add_systems(Startup, setup);

    // Update the text when the velocity or acceleration changes
    app.add_systems(
        Update,
        (
            update_acceleration,
            update_velocity_text,
            update_acceleration_text,
        ),
    );

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const VELOCITY_COLOR: Color = bevy_catppuccin::Flavor::MOCHA.green;
    const ACCELERATION_COLOR: Color = bevy_catppuccin::Flavor::MOCHA.red;
    #[cfg(feature = "debug")]
    const SCALE: f32 = 1.0;

    // Add camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

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

    // simple light
    commands.spawn(DirectionalLight::default());

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
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((Text::new("Velocity = "), TextColor(VELOCITY_COLOR.into())));

                    parent.spawn((
                        Text::new("[0, 0, 0]"),
                        TextColor(VELOCITY_COLOR.into()),
                        VelocityText,
                    ));
                });

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
                        Text::new("[0, 0, 0]"),
                        TextColor(ACCELERATION_COLOR.into()),
                        AccelerationText,
                    ));
                });
        });
}

// make accerlation depend on time with a sine wave in x and z
fn update_acceleration(mut query: Query<&mut Acceleration>, time: Res<Time>) {
    const AMPLITUDE: f32 = 5.0;

    for mut acceleration in query.iter_mut() {
        let x = AMPLITUDE * time.elapsed_secs().sin();
        let z = AMPLITUDE * time.elapsed_secs().cos();
        acceleration.value = Vec3::new(x, 0.0, z);
    }
}

fn update_velocity_text(
    query: Query<&Velocity, Changed<Velocity>>,
    mut query_text: Query<&mut Text, With<VelocityText>>,
) {
    let velocity = query.get_single().unwrap();
    let mut text = query_text.get_single_mut().unwrap();

    text.0 = format!(
        "[{:.2}, {:.2}, {:.2}]",
        velocity.value.x, velocity.value.y, velocity.value.z
    );
}

fn update_acceleration_text(
    query: Query<&Acceleration, Changed<Acceleration>>,
    mut query_text: Query<&mut Text, With<AccelerationText>>,
) {
    let acceleration = query.get_single().unwrap();
    let mut text = query_text.get_single_mut().unwrap();

    text.0 = format!(
        "[{:.2}, {:.2}, {:.2}]",
        acceleration.value.x, acceleration.value.y, acceleration.value.z
    );
}
