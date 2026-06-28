use crate::physics::consts::*;
use bevy::prelude::*;

// TODO: do we need a terminal velidity to stop it moving so fast?
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub movement_speed: f32, // Pixels per second
    pub size: Vec2,
    pub terminal_velocity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: MOVEMENT_SPEED,
            size: Vec2::new(50.0, 50.0),
            terminal_velocity: TERMINAL_VELOCITY,
        }
    }
}

// Even though player doesn't hold position information, it feels semantically accurate
// to have this logic as a method on Player.
impl Player {
    // Depth the player has fallen from the top of the map
    pub fn depth(transform: &Transform) -> f32 {
        (-transform.translation.y / (PIXELS_PER_METRE * DEPTH_SCALE)).max(0.0)
    }
}
