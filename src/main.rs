use crate::{
    background::{camera_parallax_bundle, BackgroundPlugin},
    killzone::KillZonePlugin,
    obstacles::ObstaclesPlugin,
    physics::{consts::*, PhysicsPlugin},
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

fn setup(mut commands: Commands) {
    // Camera and background
    commands.spawn(camera_parallax_bundle());

    // Player
    commands.queue_spawn_scene(player_scene());

    // Walls
    let wall_offset = (64.0 * 5.0) / 2.0;
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

fn player_scene() -> impl Scene {
    let size = Vec2::new(50.0, 50.0);

    bsn! {
        Transform
        Player
        Sprite {
            image: "textures/subject/chick.png",
            custom_size: size,
        }
        RigidBody
        Collider::rectangle(size.x, size.y)
        CollisionEventsEnabled
        // Friction disabled for now or else the player can hold onto the walls
        Friction::ZERO
        // Game crashes if Player collides with wall and rotates 💀
        LockedAxes::ROTATION_LOCKED
        // Apply a constant force of 10 N in the positive y direction (to represent air
        // resistance or something, (I just found it in the Avian docs and wanted to use
        // it (not scope creep)))
        ConstantForce::new(0.0, UPWARD_FORCE)
        // Feather falls slowly
        GravityScale(FEATHER_GRAVITATIONAL_SCALE)
    }
}

pub mod background;
pub mod killzone;
pub mod obstacles;
pub mod physics;
pub mod player;
pub mod state;
pub mod ui;
pub mod winzone;
