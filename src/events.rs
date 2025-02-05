use bevy::prelude::*;

use crate::bullet::*;
use crate::common::*;
use crate::player::*;
use crate::zombie::*;

#[derive(Default)]
pub struct ShootEvent(pub Transform);

pub fn handle_shoot_events(
    mut shoot_event_reader: EventReader<ShootEvent>,
    commands: Commands,
    texture_handles: Res<ResourceHandles>,
) {
    if let Some(event) = shoot_event_reader.iter().next_back() {
        spawn_bullet(commands, &texture_handles, event.0);
    }
}

#[derive(Default)]
pub struct PlayerMeleeEvent;

pub fn handle_player_melee_events(
    mut melee_event_reader: EventReader<PlayerMeleeEvent>,
    mut hurt_zombie_event_writer: EventWriter<HurtZombieEvent>,
    zombie_query: Query<(Entity, &HurtCollider, &Transform, &GlobalTransform), With<Zombie>>,
    hit_collider_query: Query<(&HitCollider, &GlobalTransform, &Attack), With<PlayerHitCollider>>,
) {
    if let Some(_event) = melee_event_reader.iter().last() {
        if let Some((hit_collider, hit_transform, attack)) = hit_collider_query.iter().next() {
            for (entity, hurt_collider, _hurt_transform, g_transform) in zombie_query.iter() {
                println!("Hit  (Transform: {:?}) (Radius: {})", hit_transform.translation, hit_collider.radius);
                println!("Hurt (Transform: {:?}) (Radius: {})", g_transform.translation, hurt_collider.radius);
                if circles_collide(
                        hit_collider.radius,
                        hit_transform.translation,
                        hurt_collider.radius,
                        g_transform.translation
                ) {
                    hurt_zombie_event_writer.send(HurtZombieEvent { entity, damage: attack.damage });
                }
            }
        }
    }
}

pub struct HurtZombieEvent {
    pub entity: Entity,
    pub damage: i64,
}

pub fn handle_hurt_zombie_event(
    mut hurt_zombie_event_reader: EventReader<HurtZombieEvent>,
    mut commands: Commands,
    mut zombie_query: Query<&mut Health, With<Zombie>>,
) {
    if let Some(event) = hurt_zombie_event_reader.iter().last() {
        println!("Handling hurt zombie event");
        if let Ok(mut health) = zombie_query.get_mut(event.entity) {
            health.points -= event.damage;
            if health.points <= 0 {
                commands.entity(event.entity).despawn_recursive();
            }
        }
    }
}
