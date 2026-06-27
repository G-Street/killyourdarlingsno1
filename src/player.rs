use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub movement_speed: f32, // Pixels per second
    pub size: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 500.0,
            size: Vec2::new(50.0, 50.0),
        }
    }
}
