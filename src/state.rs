use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Title,
    Playing,
    Dead,
    Won,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Title), pause_physics)
            .add_systems(OnEnter(GameState::Dead), pause_physics)
            .add_systems(OnEnter(GameState::Won), pause_physics)
            .add_systems(OnEnter(GameState::Playing), unpause_physics);
    }
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}

fn unpause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}
