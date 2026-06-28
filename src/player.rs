use bevy::prelude::*;

// TODO: do we need a terminal velidity to stop it moving so fast?
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub movement_speed: f32, // Pixels per second
    pub size: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            // TODO: is this too fast?
            movement_speed: 500.0,
            size: Vec2::new(50.0, 50.0),
        }
    }
}

// Even though player doesn't hold position information, it feels semantically accurate
// to have this logic as a method on Player.
impl Player {
    // Depth the player has fallen from the top of the map
    pub fn depth(transform: &Transform) -> f32 {
        (-transform.translation.y / (crate::physics::PIXELS_PER_METRE * 10.0)).max(0.0)
    }
}
