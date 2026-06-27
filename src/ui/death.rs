use crate::state::GameState;
use bevy::prelude::*;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), spawn_death_overlay)
            .add_systems(
                Update,
                death_overlay_system.run_if(in_state(GameState::Dead)),
            );
    }
}

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

// TODO: respawn player
fn death_overlay_system() {}
