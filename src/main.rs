use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .run();
}

// TODO: calculate velocity from constant starting velocity and player
#[derive(Component)]
struct Player {
    weight: f32,
    velocity: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.25, 0.55), Vec2::new(300.0, 200.0)),
        // Transform::from_xyz(0., 0., 0.),
        Player {
            weight: 0.1,
            velocity: 0.0,
        },
    ));
}

// TODO: this is not yet working, but I am trying Frankenstein examples from bevy
fn controls(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut transform: Query<&mut Transform>,
) {
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 1.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 1.0;
    }
}
