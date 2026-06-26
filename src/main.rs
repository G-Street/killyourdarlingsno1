use crate::{
    background::{camera_parallax_bundle, parallax_plugin},
    collider::{collition_system, Collider, Collision},
    killzone::{kill_player_system, killzone_system, KillPlayer, KillZone},
    player::Player,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
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
        )
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
    commands.spawn(camera_parallax_bundle());

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

    // TODO: this control is temporarily added to test pagination of background.  Once gravity
    //   is implemented, we can look at removing this and modifying the camera-following code
    //   (`background::move_camera`) to simply follow the Player entity.
    //
    // NOTE: on the topic of gravity, we could consider using the Avian physics engine:
    //     <github.com/avianphysics/avian/tree/d1295a30/crates/avian2d/examples>
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 1.0;
    }
}

pub mod background;
pub mod collider;
pub mod killzone;
pub mod player;
