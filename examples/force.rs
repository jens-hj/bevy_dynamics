use bevy::{
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::*,
};
use dynamics::{Damping, DynamicsPlugin, Mass, Velocity};
#[cfg(feature = "debug")]
use dynamics::{Debug, DebugColors, DebugScale};

mod common;
use common::*;

const TICK_RATE: f64 = 100.0;

/// Bevy [`Resource`] that holds the force arrow.
#[derive(Resource)]
pub struct ForceArrow {
    /// The start position of the force arrow.
    pub start: Option<Vec3>,
    /// The end position of the force arrow.
    pub end: Option<Vec3>,
    /// The color of the force arrow.
    pub color: Color,
}

impl Default for ForceArrow {
    /// Initializes the force arrow without a start or end position and a color
    /// of mauve (purple) from the Catppuccin color palette.
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            color: bevy_catppuccin::Flavor::MOCHA.mauve,
        }
    }
}

impl ForceArrow {
    /// Returns the vector of the force arrow.
    pub fn vector(&self) -> Option<Vec3> {
        let Some(start) = self.start else {
            return None;
        };

        let Some(end) = self.end else {
            return None;
        };

        Some(end - start)
    }

    /// Resets the force arrow by clearing the start and end positions.
    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
    }
}

fn main() {
    let mut app = App::new();
    // Determine the fixed update rate
    app.insert_resource(Time::<Fixed>::from_hz(TICK_RATE));
    app.add_plugins((DefaultPlugins, MeshPickingPlugin, DynamicsPlugin));

    // Setup the visuals
    app.insert_resource(ClearColor(bevy_catppuccin::Flavor::MOCHA.base));
    app.insert_resource(AmbientLight {
        brightness: 500.0,
        ..default()
    });

    // Setup the force arrow that shows up when dragging
    app.init_resource::<ForceArrow>();

    // Add a simple entity with velocity, acceleration, and debug
    app.add_systems(Startup, (setup_scene, setup_text, setup_dragging_plane));

    // Update the text when the velocity or acceleration changes
    app.add_systems(
        Update,
        (
            update_velocity_text,
            update_acceleration_text,
            draw_force_arrow,
        ),
    );

    app.run();
}

/// Bevy [`Startup`] system that sets up the scene with a camera and a dynamics
/// entity with a mass of 1.0, an initial velocity of [0, 0, 0], an initial
/// acceleration of [0, 0, 0], and a damping of 0.05.
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
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(
                bevy_catppuccin::Flavor::MOCHA.blue,
            ))),
            Transform::from_xyz(-1.0, 0.0, 0.0),
            Mass::new(1.0),
            Damping::new(0.5),
            PickingBehavior::IGNORE,
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

    // Add moving entity with debug visualization
    let entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(
                bevy_catppuccin::Flavor::MOCHA.peach,
            ))),
            Transform::from_xyz(1.0, 0.0, 0.0),
            Mass::new(2.0),
            Damping::new(0.5),
            PickingBehavior::IGNORE,
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

/// Bevy [`Startup`] system that sets up a plane that can observe pointer
/// events.
fn setup_dragging_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(
                bevy_catppuccin::Flavor::MOCHA.green.with_alpha(0.0),
            ))),
        ))
        .observe(pointer_drag_start)
        .observe(pointer_drag)
        .observe(pointer_drag_end);
}

/// Bevy [`Observer`] that initializes the force arrow when the pointer starts
/// dragging on the plane.
///
/// E.g. when the user clicks, holds and starts dragging the pointer over the
/// plane
fn pointer_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut force_arrow: ResMut<ForceArrow>,
) {
    force_arrow.start = trigger.event.hit.position;
    force_arrow.end = trigger.event.hit.position;
}

/// Bevy [`Observer`] that updates the starting position of the force arrow
/// while the pointer is dragging on the plane.
fn pointer_drag(
    trigger: Trigger<Pointer<Drag>>,
    mut force_arrow: ResMut<ForceArrow>,
) {
    force_arrow.start = Some(
        force_arrow.start.unwrap()
            - trigger.event.delta.extend(0.0).xzy() / 200.0,
    );
}

/// Bevy [`Observer`] that applies the total force to the entity when the
/// pointer stops dragging on the plane.
///
/// E.g. when the user stop holding the mouse button.
fn pointer_drag_end(
    _trigger: Trigger<Pointer<DragEnd>>,
    mut force_arrow: ResMut<ForceArrow>,
    mut query: Query<(&Mass, &mut Velocity)>,
) {
    let Some(vector) = force_arrow.vector() else {
        return;
    };

    for (mass, mut velocity) in query.iter_mut() {
        velocity.apply_impulse(vector * 50.0, &mass, 1.0 / TICK_RATE as f32);
    }

    force_arrow.reset();
}

/// Bevy [`System`] that draws the force arrow in the scene with the Bevy
/// [`Gizmos`].
fn draw_force_arrow(mut gizmos: Gizmos, force_arrow: Res<ForceArrow>) {
    let Some(start) = force_arrow.start else {
        return;
    };

    let Some(end) = force_arrow.end else {
        return;
    };

    gizmos.arrow(start, end, force_arrow.color);
}
