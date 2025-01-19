use bevy::{
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::*,
};
use dynamics::{DynamicsPlugin, Mass, Velocity};

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
            color: bevy_catppuccin::Flavor::MOCHA.surface1,
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

    // Setup the particles
    let particles = vec![
        ParticleConfiguration {
            name: "Light".into(),
            mass: 1.0,
            radius: 0.1,
            color: bevy_catppuccin::Flavor::MOCHA.blue,
            position: Vec3::new(1.0, 0.0, 0.0),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            damping: 0.5,
        },
        ParticleConfiguration {
            name: "Heavy".into(),
            mass: 3.0,
            radius: 0.3,
            color: bevy_catppuccin::Flavor::MOCHA.mauve,
            position: Vec3::new(-1.0, 0.0, 0.0),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            damping: 0.5,
        },
    ];
    app.insert_resource(SpawnConfiguration::new(particles));

    // Add a simple entity with velocity, acceleration, and debug
    app.add_systems(
        Startup,
        (
            setup_scene,
            setup_text,
            setup_instructions,
            setup_dragging_plane,
        ),
    );

    // Update the text when the velocity or acceleration changes
    app.add_systems(
        Update,
        (
            update_velocity_text,
            update_acceleration_text,
            draw_force_arrows,
        ),
    );

    app.run();
}

/// Bevy [`Startup`] system that sets up the instructions text.
fn setup_instructions(mut commands: Commands) {
    commands
        .spawn(Node {
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_child((
            Text::new("Drag to apply force to the particles"),
            TextColor(bevy_catppuccin::Flavor::MOCHA.text),
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
            - trigger.event.delta.extend(0.0).xzy() / 175.0,
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
fn draw_force_arrows(
    mut gizmos: Gizmos,
    force_arrow: Res<ForceArrow>,
    query: Query<(&Transform, &ParticleConfiguration), With<Mass>>,
) {
    let Some(start) = force_arrow.start else {
        return;
    };

    let Some(end) = force_arrow.end else {
        return;
    };

    for (transform, config) in query.iter() {
        let dir = (end - start).normalize();

        let start = transform.translation + start - end - dir * config.radius;
        let end = transform.translation - dir * config.radius;

        gizmos.arrow(start, end, config.color.with_alpha(0.5));
    }

    // Also draw the force where it is being dragged
    gizmos.arrow(start, end, force_arrow.color);
}
