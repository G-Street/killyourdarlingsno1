use crate::killzone::KillZone;
use avian2d::collision::{collider::Collider, collision_events::CollisionEventsEnabled};
use bevy::prelude::*;
use chacha20::ChaCha8Rng;
use rand::{RngExt, SeedableRng};

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObstacleLanes(vec![-100.0, 0.0, 100.0]))
            .insert_resource(ObstacleSpacing {
                starup_grace: -300.0,
                spawn_grace: -500.0,
                spacing: -500.0,
            })
            .add_systems(Startup, startup)
            .add_systems(FixedUpdate, (spawn_obstacles, despawn_obstacles));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Random(ChaCha8Rng);

#[derive(Resource, Deref)]
struct ObstacleLanes(Vec<f32>);

#[derive(Resource)]
struct ObstacleSpacing {
    /// The distance before the first obstacle should appear
    starup_grace: f32,

    /// The distance from the player the obstacles should spawn
    spawn_grace: f32,

    /// The space between obstacles
    spacing: f32,
}

fn startup(mut commands: Commands) {
    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    commands.insert_resource(Random(seeded_rng));
}

fn spawn_obstacles(
    mut commands: Commands,
    mut previous: Local<f32>,
    camera: Single<&Transform, With<Camera>>,
    spacing: Res<ObstacleSpacing>,
    lanes: Res<ObstacleLanes>,
    mut random: ResMut<Random>,
) {
    if *previous > spacing.starup_grace {
        *previous = spacing.starup_grace;
    }

    if camera.translation.y > spacing.spacing + *previous {
        return;
    }

    *previous += spacing.spacing;

    let lane = lanes.get(random.random_range(0..2)).unwrap();

    commands.spawn(enemy_bundle(Vec2::new(
        *lane,
        spacing.spawn_grace + *previous,
    )));
}

fn despawn_obstacles(mut _commands: Commands) {}

pub fn enemy_bundle(pos: Vec2) -> impl Bundle {
    (
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Sprite {
            color: Color::srgb(0.55, 0.25, 0.25),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Collider::rectangle(50.0, 50.0),
        CollisionEventsEnabled,
        KillZone,
    )
}
