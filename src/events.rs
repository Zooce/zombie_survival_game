use bevy::prelude::*;

use crate::bullet::*;
use crate::common::*;
use crate::player::*;
use crate::zombie::*;

#[derive(Default)]
pub struct ShootEvent(pub Transform);

pub fn handle_shoot_events(
    shoot_events: Res<Events<ShootEvent>>,
    mut shoot_events_reader: Local<EventReader<ShootEvent>>,
    commands: &mut Commands,
    texture_handles: Res<ResourceHandles>,
) {
    if let Some(event) = shoot_events_reader.iter(&shoot_events).next_back() {
        spawn_bullet(commands, &texture_handles, event.0);
    }
}

#[derive(Default)]
pub struct PlayerMeleeEvent;

pub fn handle_player_melee_events(
    melee_events: Res<Events<PlayerMeleeEvent>>,
    mut melee_events_reader: Local<EventReader<PlayerMeleeEvent>>,
    mut hurt_zombie_events: ResMut<Events<HurtZombieEvent>>,
    zombie_query: Query<(Entity, &HurtCollider, &Transform), With<Zombie>>,
    hit_collider_query: Query<(&HitCollider, &Transform, &Attack), With<Player>>,
) {
    if let Some(_event) = melee_events_reader.latest(&melee_events) {
        if let Some((hit_collider, hit_transform, attack)) = hit_collider_query.iter().next() {
            println!("FWD: {}", hit_transform.forward());
            let p1 = hit_transform.translation + hit_collider.transform.translation;
            for (entity, hurt_collider, hurt_transform) in zombie_query.iter() {
                let p2 = hurt_transform.translation + hurt_collider.offset;
                println!("P1: {} ({})", p1, hit_transform.translation);
                println!("P2: {}", p2);
                if circles_collide(hit_collider.radius, p1, hurt_collider.radius, p2) {
                    hurt_zombie_events.send(HurtZombieEvent { entity, damage: attack.damage });
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
    hurt_zombie_events: Res<Events<HurtZombieEvent>>,
    mut hurt_zombie_event_reader: Local<EventReader<HurtZombieEvent>>,
    commands: &mut Commands,
    mut zombie_query: Query<&mut Health, With<Zombie>>,
) {
    if let Some(event) = hurt_zombie_event_reader.latest(&hurt_zombie_events) {
        println!("Handling hurt zombie event");
        if let Ok(mut health) = zombie_query.get_mut(event.entity) {
            health.points -= event.damage;
            if health.points <= 0 {
                commands.despawn_recursive(event.entity);
            }
        }
    }
}
