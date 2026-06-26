use crate::player::Player;
use bevy::prelude::*;
use bevy_parallaxium::{
    LayerRepeat, ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems,
};

pub fn parallax_plugin(app: &mut App) {
    app.add_plugins(ParallaxPlugin)
        .add_systems(Update, move_camera.before(ParallaxSystems));
}

// TODO: is it idiomatic to ship a whole bundle from a submodule, or is it best to construct
//   this in the main module on setup?  I just didn't want to import all of the bevy parallax
//   stuff again in main.rs.
pub fn camera_parallax_bundle() -> impl Bundle {
    (
        Camera2d,
        ParallaxCamera::default(),
        children![
            ParallaxLayer::new("textures/wall_brick_sand_center.png", 0.0)
                .with_tile_size(UVec2::new(64, 128))
                .with_scale(Vec2::splat(5.0))
                .with_repeat(LayerRepeat::vertical())
                .with_z(-1.0),
        ],
    )
}

// Camera should move with player entity
fn move_camera(
    mut last_pos: Local<Vec2>,
    mut move_events: MessageWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        error!("player not found");
        return;
    };

    let current_pos = player_transform.translation.truncate();
    let translation = current_pos - *last_pos;
    *last_pos = current_pos;

    // Apply translation to the camera, which thereby shifts the parallax background
    if translation != Vec2::ZERO {
        let camera = camera_query.single().unwrap();
        move_events.write(ParallaxMoveEvent::translate(camera, translation));
    }
}
