use bevy::prelude::*;
use crate::common::*;

pub struct Player;
pub struct PlayerHitCollider;

pub fn spawn_player(
    commands: &mut Commands,
    resource_handles: &ResourceHandles,
) {
    let offset = Transform::from_translation(Vec3::new(32.0, 0.0, 0.0));
    // basic player components
    commands.spawn()
        // .with_bundle(SpriteSheetBundle {
        //     texture_atlas: resource_handles.player_texture_atlas_handle.clone(),
        //     transform: Transform::from_scale(Vec3::splat(6.0)),
        //     ..Default::default()
        // })
        .insert_bundle(SpriteBundle {
            material: resource_handles.player_handle.clone(),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.08, true))
        // .with(AnimationState{ dir_offset: 6, frame: 0, is_walking: false })
        .insert(Player)
        // debugging
        .with_children(|parent| {
            parent.spawn()
                .insert_bundle(SpriteBundle {
                    material: resource_handles.debug_hit_collider_handle.clone(),
                    transform: offset,
                    ..Default::default()
                })
                .insert(HitCollider {
                    radius: 16.0,
                    transform: offset,
                })
                .insert(Attack {
                    damage: 30,
                })
                .insert(PlayerHitCollider)
                ;
        })
        ;
}
