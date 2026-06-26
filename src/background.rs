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

fn move_camera(
    input: Res<ButtonInput<KeyCode>>,
    mut move_events: MessageWriter<ParallaxMoveEvent>,
    query: Query<Entity, With<Camera>>,
) {
    let camera = query.single().unwrap();
    let mut translation = Vec2::ZERO;

    // TODO: currently the background is changing when the down arrow is pressed.
    //   In the actual game, we will want the background to be dynamic with the camera.
    //   Also, we will want the camera to be slightly lower down, below the Player,
    //   so that the user can see what is coming up.
    //
    //   See example from bevy parallax crate:
    //     <github.com/Gialale-Games/bevy_parallaxium/blob/f18b3524/examples/test.rs>
    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        translation.y -= 1.0;
    }

    // Apply translation to parallax background
    if translation != Vec2::ZERO {
        move_events.write(ParallaxMoveEvent::translate(camera, translation));
    }
}
