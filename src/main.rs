use crate::{
    background::{camera_parallax_bundle, parallax_plugin},
    killzone::{kill_player_system, killzone_system, KillPlayer},
    obstacles::ObstaclesPlugin,
    player::Player,
};
use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Feather Falling IV".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(
                    // nearest sampling, to prevent blurry sprites
                    ImagePlugin::default_nearest(),
                ),
            // Use units-per-meter scaling factor of 1 meter to 20 pixels
            PhysicsPlugins::default().with_length_unit(20.0),
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 10.0))
        .add_message::<KillPlayer>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .add_systems(FixedUpdate, (killzone_system, kill_player_system).chain())
        .add_plugins((parallax_plugin, ObstaclesPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera and background
    commands.spawn(camera_parallax_bundle());

    // Player
    let player = Player::default();
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        player,
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.55),
            custom_size: Some(player.size),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(player.size.x, player.size.y),
        CollisionEventsEnabled,
        // Friction disabled for now or else the player can hold onto the walls
        Friction::ZERO,
        // Game crashes if Player collides with wall and rotates 💀
        LockedAxes::ROTATION_LOCKED,
    ));

    // Walls
    let wall_offset = (background::WIDTH as f32) / 2.0;
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(0.0, f32::MAX),
        Transform::from_xyz(-wall_offset, 0.0, 0.0),
        // Friction disabled for now or else the player can hold onto the walls
        Friction::ZERO,
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(0.0, f32::MAX),
        Transform::from_xyz(wall_offset, 0.0, 0.0),
        // Friction disabled for now or else the player can hold onto the walls
        Friction::ZERO,
    ));
}

fn controls(input: Res<ButtonInput<KeyCode>>, query: Single<(&Player, &mut LinearVelocity)>) {
    let (player, mut velocity) = query.into_inner();

    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.x = -player.movement_speed;
    } else if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.x = player.movement_speed;
    } else {
        velocity.x = 0.0;
    }
}

pub mod background;
pub mod killzone;
pub mod obstacles;
pub mod player;
