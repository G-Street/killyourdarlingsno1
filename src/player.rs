use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 5.0,
        }
    }
}
