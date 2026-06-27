use crate::{player::Player, ui::death::spawn_death_overlay};
use avian2d::collision::collision_events::CollisionStart;
use bevy::prelude::*;

pub struct KillZonePlugin;

impl Plugin for KillZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<KillPlayer>()
            .add_systems(FixedUpdate, (killzone_system, kill_player_system).chain());
    }
}

#[derive(Component)]
pub struct KillZone;

#[derive(Message, Deref)]
pub struct KillPlayer(Entity);

fn killzone_system(
    mut collisions: MessageReader<CollisionStart>,
    players: Query<&Player>,
    killzones: Query<&KillZone>,
    mut events: MessageWriter<KillPlayer>,
) {
    for c in collisions.read() {
        info!("collision: {:?} - {:?}", c.collider1, c.collider2);
        let p1 = players.get(c.collider1);
        let p2 = players.get(c.collider2);
        if p1.is_err() == p2.is_err() {
            continue;
        }

        let k1 = killzones.get(c.collider1);
        let k2 = killzones.get(c.collider2);
        if k1.is_err() == k2.is_err() {
            continue;
        }

        if p1.is_ok() && k2.is_ok() {
            events.write(KillPlayer(c.collider1));
            continue;
        }

        if p2.is_ok() && k1.is_ok() {
            events.write(KillPlayer(c.collider2));
            continue;
        }
    }
}

fn kill_player_system(
    mut commands: Commands,
    mut events: MessageReader<KillPlayer>,
    players: Query<&Player>,
) {
    for e in events.read() {
        if players.get(e.entity()).is_err() {
            error!("player not found");
            continue;
        };

        commands.entity(e.entity()).despawn();
        commands.run_system_cached(spawn_death_overlay);
    }
}
