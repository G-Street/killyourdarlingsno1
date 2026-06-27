use crate::state::GameState;
use bevy::prelude::*;

pub struct WonPlugin;

impl Plugin for WonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Won), spawn_won_overlay)
            .add_systems(Update, won_overlay_system.run_if(in_state(GameState::Won)));
    }
}

#[derive(Component)]
pub struct WonOverlay;

pub fn spawn_won_overlay(mut commands: Commands) {
    commands
        .spawn((
            WonOverlay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("you made it to the bottom :)"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}

// TODO: option to respawn player or return to title screen?
// TODO: Once bottom is reached, we should probably stop showing obsacles and start update
//       the background to represent "the end" (maybe like the feather is trying to get back
//       to its nest to a derpy sewer pigeon or something).
fn won_overlay_system() {}
