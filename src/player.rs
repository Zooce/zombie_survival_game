use bevy::prelude::*;
use crate::common::*;

pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    resource_handles: &ResourceHandles,
) {
    let offset = Transform::from_translation(Vec3::new(32.0, 0.0, 0.0));
    // basic player components
    commands
        // .spawn(SpriteSheetBundle {
        //     texture_atlas: resource_handles.player_texture_atlas_handle.clone(),
        //     transform: Transform::from_scale(Vec3::splat(6.0)),
        //     ..Default::default()
        // })
        .spawn(SpriteBundle {
            material: resource_handles.player_handle.clone(),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.08, true))
        // .with(AnimationState{ dir_offset: 6, frame: 0, is_walking: false })
        .with(Player)
        .with(Attack {
            damage: 30,
        })
        .with(HitCollider {
            radius: 16.0,
            transform: offset.clone(),
        })
        // debugging
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    material: resource_handles.debug_hit_collider_handle.clone(),
                    transform: offset,
                    ..Default::default()
                })
                ;
        })
        ;
}
