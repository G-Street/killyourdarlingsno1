use crate::{player::Player, state::GameState};
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(Update, update_depth.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct DepthText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        DepthText,
        Text::new("Depth: 0.0 m"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            right: Val::Px(16.0),
            ..default()
        },
    ));
}

fn update_depth(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Text, With<DepthText>>,
) {
    let Ok(transform) = player_query.single() else {
        error!("player not found");
        return;
    };

    let Ok(mut text) = text_query.single_mut() else {
        error!("text not found");
        return;
    };

    let depth = Player::depth(transform);
    **text = format!("Depth: {:.2} m", depth);
}
