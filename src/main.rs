use crate::{
    background::{camera_parallax_bundle, parallax_plugin},
    collider::{collition_system, Collider, Collision},
    killzone::{kill_player_system, killzone_system, KillPlayer, KillZone},
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
        .add_message::<Collision>()
        .add_message::<KillPlayer>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .add_systems(
            FixedUpdate,
            (collition_system, killzone_system, kill_player_system).chain(),
        )
        .add_plugins(parallax_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera and background
    commands.spawn(camera_parallax_bundle());

    // Player
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.55),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Player {
            movement_speed: 5.0,
        },
        Collider {
            size: Vec2::new(50.0, 50.0),
        },
        RigidBody::Dynamic,
    ));

    // Enemy
    commands.spawn((
        Transform::from_xyz(350.0, 0.0, 0.0),
        Sprite {
            color: Color::srgb(0.55, 0.25, 0.25),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Collider {
            size: Vec2::new(50.0, 50.0),
        },
        KillZone,
    ));
}

fn controls(input: Res<ButtonInput<KeyCode>>, query: Single<(&Player, &mut Transform)>) {
    let (player, mut transform) = query.into_inner();

    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        transform.translation.x -= player.movement_speed;
    }

    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        transform.translation.x += player.movement_speed;
    }
}

pub mod background;
pub mod collider;
pub mod killzone;
pub mod player;
