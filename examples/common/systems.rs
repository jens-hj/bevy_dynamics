use bevy::prelude::*;
use dynamics::{Acceleration, Velocity};

use super::{
    components::{
        AccelerationMagnitudeText, AccelerationVectorText,
        VelocityMagnitudeText, VelocityVectorText,
    },
    constants::{ACCELERATION_COLOR, VELOCITY_COLOR},
};

/// Bevy [`Startup`] system that creates the text nodes for the velocity and
/// acceleration.
pub fn setup_text(mut commands: Commands) {
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
            // Showing the velocity as "Velocity = |[v.x, v.y, v.z]| = ||v||"
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
                        Text::new("|[0, 0, 0]|"),
                        TextColor(VELOCITY_COLOR.into()),
                        VelocityVectorText,
                    ));

                    parent.spawn((
                        Text::new(" = "),
                        TextColor(VELOCITY_COLOR.into()),
                    ));

                    parent.spawn((
                        Text::new("0"),
                        TextColor(VELOCITY_COLOR.into()),
                        VelocityMagnitudeText,
                    ));
                });

            // Showing the acceleration as "Acceleration = |[a.x, a.y, a.z]| =
            // ||a||"
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
                        Text::new("|[0, 0, 0]|"),
                        TextColor(ACCELERATION_COLOR.into()),
                        AccelerationVectorText,
                    ));

                    parent.spawn((
                        Text::new(" = "),
                        TextColor(ACCELERATION_COLOR.into()),
                    ));

                    parent.spawn((
                        Text::new("0"),
                        TextColor(ACCELERATION_COLOR.into()),
                        AccelerationMagnitudeText,
                    ));
                });
        });
}

/// Bevy [`Update`] system that updates the velocity text.
pub fn update_velocity_text(
    query: Query<&Velocity>,
    mut query_vector_text: Query<
        &mut Text,
        (With<VelocityVectorText>, Without<VelocityMagnitudeText>),
    >,
    mut query_magnitude_text: Query<
        &mut Text,
        (With<VelocityMagnitudeText>, Without<VelocityVectorText>),
    >,
) {
    let Ok(velocity) = query.get_single() else {
        return;
    };

    if let Ok(mut vector_text) = query_vector_text.get_single_mut() {
        vector_text.0 = format!(
            "|[{:.2}, {:.2}, {:.2}]|",
            velocity.value.x, velocity.value.y, velocity.value.z
        );
    };

    if let Ok(mut magnitude_text) = query_magnitude_text.get_single_mut() {
        magnitude_text.0 = format!("{:.2}", velocity.speed());
    };
}

/// Bevy [`Update`] system that updates the acceleration text.
pub fn update_acceleration_text(
    query: Query<&Acceleration>,
    mut query_vector_text: Query<
        &mut Text,
        (
            With<AccelerationVectorText>,
            Without<AccelerationMagnitudeText>,
        ),
    >,
    mut query_magnitude_text: Query<
        &mut Text,
        (
            With<AccelerationMagnitudeText>,
            Without<AccelerationVectorText>,
        ),
    >,
) {
    let Ok(acceleration) = query.get_single() else {
        return;
    };

    if let Ok(mut vector_text) = query_vector_text.get_single_mut() {
        vector_text.0 = format!(
            "|[{:.2}, {:.2}, {:.2}]|",
            acceleration.value.x, acceleration.value.y, acceleration.value.z
        );
    };

    if let Ok(mut magnitude_text) = query_magnitude_text.get_single_mut() {
        magnitude_text.0 = format!("{:.2}", acceleration.magnitude());
    };
}
