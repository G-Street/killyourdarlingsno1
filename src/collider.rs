use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

impl Collider {
    fn aabb(&self, translation: &Vec2) -> Aabb2d {
        Aabb2d::new(*translation, self.half_size())
    }

    fn half_size(&self) -> Vec2 {
        Vec2::new(self.size.x / 2.0, self.size.y / 2.0)
    }
}

#[derive(Message)]
pub struct Collision(pub Entity, pub Entity);

pub fn collition_system(
    colliders: Query<(Entity, &Collider, &Transform)>,
    mut collisions: MessageWriter<Collision>,
) {
    for [(e0, c0, t0), (e1, c1, t1)] in colliders.iter_combinations() {
        let aabb0 = c0.aabb(&t0.translation.xy());
        let aabb1 = c1.aabb(&t1.translation.xy());
        if !aabb0.intersects(&aabb1) {
            continue;
        }

        collisions.write(Collision(e0, e1));
    }
}
