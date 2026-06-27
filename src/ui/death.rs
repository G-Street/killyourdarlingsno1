use bevy::prelude::*;

#[derive(Component)]
pub struct DeathOverlay;

pub fn spawn_death_overlay(mut commands: Commands) {
    commands
        .spawn((
            DeathOverlay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("you died :("),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
            // TODO: respawn button
        });
}

pub fn death_overlay_system(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    overlay_query: Query<Entity, With<DeathOverlay>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            for entity in &overlay_query {
                commands.entity(entity).despawn();
            }
            // TODO: respawn player
        }
    }
}
