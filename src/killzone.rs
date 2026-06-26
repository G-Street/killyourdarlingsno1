use crate::collider::Collision;
use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct KillZone;

#[derive(Message, Deref)]
pub struct KillPlayer(Entity);

pub fn killzone_system(
    mut collisions: MessageReader<Collision>,
    players: Query<&Player>,
    killzones: Query<&KillZone>,
    mut events: MessageWriter<KillPlayer>,
) {
    for c in collisions.read() {
        let p0 = players.get(c.0);
        let p1 = players.get(c.1);
        if p0.is_err() == p1.is_err() {
            continue;
        }

        let k0 = killzones.get(c.0);
        let k1 = killzones.get(c.1);
        if k0.is_err() == k1.is_err() {
            continue;
        }

        if p0.is_ok() && k1.is_ok() {
            events.write(KillPlayer(c.0));
            continue;
        }

        if p1.is_ok() && k0.is_ok() {
            events.write(KillPlayer(c.1));
            continue;
        }
    }
}

pub fn kill_player_system(
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
    }
}
