use crate::{player::Player, state::GameState};
use bevy::prelude::*;

// TODO: is it deep enough?  (said the actress to the bishop)
pub const WIN_DEPTH_METRES: f32 = 50.0;

pub struct WinZonePlugin;

impl Plugin for WinZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            winzone_system.run_if(in_state(GameState::Playing)),
        );
    }
}

fn winzone_system(
    player_query: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(transform) = player_query.single() else {
        error!("player not found");
        return;
    };

    // TODO: consider a high score rather than
    // TODO (scope creep): have different "levels" with checkpoints and different kinds of
    //      obstacles
    if Player::depth(transform) >= WIN_DEPTH_METRES {
        next_state.set(GameState::Won);
    }
}
