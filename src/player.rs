use bevy::prelude::*;

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
        .with(Timer::from_seconds(0.1, true))
        .with(Player)
        ;
}

// pub AnimationState {

// }

pub fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            println!("sprite index: {}", sprite.index);
        }
    }
}
