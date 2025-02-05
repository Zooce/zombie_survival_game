use bevy::prelude::*;

use crate::common::*;
use crate::events::*;
use crate::zombie::*;

pub struct Bullet {
    decay_timer: Timer,
}

pub struct BulletSpawnInfo {
    pub transform: Transform,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            decay_timer: Timer::from_seconds(0.25, false),
        }
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    resource_handles: &ResourceHandles,
    transform: Transform,
) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            material: resource_handles.bullet_handle.clone(),
            transform,
            ..Default::default()
        })
        .insert(Bullet::default())
        .insert(HitCollider {
            radius: 8.0,
            transform,
        })
        .insert(Attack{ damage: 10 })

        // debugging
        .with_children(|parent| {
            parent.spawn()
                .insert_bundle(SpriteBundle {
                    material: resource_handles.debug_hit_collider_handle.clone(),
                    ..Default::default()
                })
                ;
        })
        ;
}

pub fn bullet_movement(
    time: Res<Time>,
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
) {
    // TODO: store this speed with either the bullet or the weapon (it will be
    //       different for each type of weapon and/or bullet)?
    let speed = 1500.0 * time.delta_seconds();

    for mut bullet_transform in bullet_query.iter_mut() {
        let (axis, mut angle) = bullet_transform.rotation.to_axis_angle();
        angle *= axis.z;
        bullet_transform.translation += Vec3::new(angle.cos(), angle.sin(), 0.0) * speed;
    }
}

pub fn bullet_decay(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in bullet_query.iter_mut() {
        if bullet.decay_timer.tick(time.delta()).finished() {
            //commands.entity(entity).despawn();
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_bullet_collisions(
    mut hurt_zombie_event_writer: EventWriter<HurtZombieEvent>,
    mut commands: Commands,
    mut zombie_query: Query<(Entity, &HurtCollider, &Transform), With<Zombie>>,
    bullet_query: Query<(Entity, &HitCollider, &Transform, &Attack), With<Bullet>>,
) {
    for (be, br, bt, attack) in bullet_query.iter() {
        for (ze, zr, zt) in zombie_query.iter_mut() {
            if circles_collide(br.radius, bt.translation, zr.radius, zt.translation) {
                // todo list:
                //  - play "got shot" sound effect
                //  - display blood splatter on the ground (this also needs to time out...?)
                //  - maybe move these different parts to different systems?
                //  - play some kind of animation on the zombie, showing that it took damage
                hurt_zombie_event_writer.send(HurtZombieEvent { entity: ze, damage: attack.damage });
                commands.entity(be).despawn_recursive();
            }
        }
    }
}
