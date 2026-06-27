use crate::player::Player;
use bevy::prelude::*;
use bevy_parallaxium::{
    LayerRepeat, ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems,
};

pub const WIDTH: u32 = 64 * 5;
pub const HEIGHT: u32 = 128 * 5;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParallaxPlugin)
            .add_systems(Update, move_camera.before(ParallaxSystems));
    }
}

// TODO: is it idiomatic to ship a whole bundle from a submodule, or is it best to construct
//   this in the main module on setup?  I just didn't want to import all of the bevy parallax
//   stuff again in main.rs.
pub fn camera_parallax_bundle() -> impl Bundle {
    (
        Camera2d,
        ParallaxCamera::default(),
        children![
            // This image is scaled up by 5×:
            //     magick input.png -filter point -resize 500% output.png
            //
            // Using `.with_scale()` caused a different bug, so we have to do it like this
            ParallaxLayer::new("textures/wall_brick_sand_center.png", 0.0)
                .with_tile_size(UVec2::new(WIDTH, HEIGHT))
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

    // We only care about vertical change, as the camera should be horizontally centred
    let translation = Vec2::Y * (current_pos.y - last_pos.y);
    *last_pos = current_pos;

    // Apply translation to the camera, which thereby shifts the parallax background
    if translation != Vec2::ZERO {
        let camera = camera_query.single().unwrap();
        move_events.write(ParallaxMoveEvent::translate(camera, translation));
    }
}
