use crate::{
    background::{camera_parallax_bundle, BackgroundPlugin},
    killzone::KillZonePlugin,
    obstacles::ObstaclesPlugin,
    physics::PhysicsPlugin,
    player::Player,
    state::GameStatePlugin,
    ui::{death::DeathPlugin, hud::HudPlugin, title::TitlePlugin, won::WonPlugin},
    winzone::WinZonePlugin,
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
                    // Nearest sampling, to prevent blurry sprites
                    ImagePlugin::default_nearest(),
                ),
            PhysicsPlugin,
            GameStatePlugin,
            TitlePlugin,
            BackgroundPlugin,
            ObstaclesPlugin,
            KillZonePlugin,
            HudPlugin,
            DeathPlugin,
            WonPlugin,
            WinZonePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera and background
    commands.spawn(camera_parallax_bundle());

    // Player
    let player = Player::default();
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        player,
        Sprite {
            image: asset_server.load("textures/subject/chick.png"),
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
        // Apply a constant force of 10 N in the positive y direction (to represent air
        // resistance or something, (I just found it in the Avian docs and wanted to use
        // it (not scope creep)))
        ConstantForce::new(0.0, 10.0),
        // Feather falls slowly
        GravityScale(0.05),
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


pub mod background;
pub mod killzone;
pub mod obstacles;
pub mod physics;
pub mod player;
pub mod state;
pub mod ui;
pub mod winzone;
