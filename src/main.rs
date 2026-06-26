use crate::{
    collider::{collition_system, Collider, Collision},
    killzone::{kill_player_system, killzone_system, KillPlayer, KillZone},
    player::Player,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Feather Falling IV".into(),
                ..default()
            }),
            ..default()
        }))
        .add_message::<Collision>()
        .add_message::<KillPlayer>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .add_systems(
            FixedUpdate,
            (collition_system, killzone_system, kill_player_system).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.55),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Player,
        Collider {
            size: Vec2::new(50.0, 50.0),
        },
    ));

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
    let (_player, mut transform) = query.into_inner();

    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 1.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 1.0;
    }
}

pub mod collider;
pub mod killzone;
pub mod player;
