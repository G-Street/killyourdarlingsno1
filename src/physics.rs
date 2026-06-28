use crate::player::Player;
use avian2d::prelude::*;
use bevy::prelude::*;
use consts::*;

pub mod consts {
    // Units-per-metre scaling factor of 1 metre to 20 pixels
    pub const PIXELS_PER_METRE: f32 = 20.0;
    pub const GRAVITATIONAL_CONSTANT: f32 = 980.0;
    pub const FEATHER_GRAVITATIONAL_SCALE: f32 = 0.05;
    pub const UPWARD_FORCE: f32 = 10.0;
    pub const MOVEMENT_SPEED: f32 = 500.0;
    // The larger the depth scale, the more pixels the player has to fall for each metre
    // of depth
    pub const DEPTH_SCALE: f32 = 10.0;
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METRE))
            .insert_resource(Gravity(Vec2::NEG_Y * GRAVITATIONAL_CONSTANT))
            .add_systems(FixedUpdate, player_controls);
    }
}

fn player_controls(
    input: Res<ButtonInput<KeyCode>>,
    query: Single<(&Player, &mut LinearVelocity)>,
) {
    let (player, mut velocity) = query.into_inner();

    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.x = -player.movement_speed;
    } else if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.x = player.movement_speed;
    } else {
        velocity.x = 0.0;
    }
}
