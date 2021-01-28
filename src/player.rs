use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::animation::*;
use crate::common::*;

pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    resource_handles: &ResourceHandles,
) {
    // basic player components
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: resource_handles.player_texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.08, true))
        .with(AnimationState{ dir_offset: 6, frame: 0, is_walking: false })
        .with(Player)
        .with(ColliderRadius(4.0))
        // debugging
        .with_children(|parent| {
            let collider_shape = shapes::Circle {
                radius: 4.0,
                ..Default::default()
            };

            parent.spawn(ShapeBuilder::build_as(
                &collider_shape,
                resource_handles.debug_collider_handle.clone(),
                TessellationMode::Stroke(StrokeOptions::default()),
                Transform::default())
            );
        })
        ;
}
