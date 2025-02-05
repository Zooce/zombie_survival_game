use bevy::prelude::*;

use crate::player::*;

pub const PLAYER_ANIM_FRAME_COUNT: u32 = 6;
pub const PLAYER_ANIM_DIR_COUNT: u32 = 8;

pub struct AnimationState {
    pub dir_offset: u32,
    pub frame: u32,
    pub is_walking: bool,
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut AnimationState), With<Player>>
) {
    for (mut timer, mut sprite, mut anim_state) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if anim_state.is_walking {
                anim_state.frame = (anim_state.frame + 1) % PLAYER_ANIM_FRAME_COUNT;
            } else {
                anim_state.frame = 0;
            }
            sprite.index = anim_state.frame + anim_state.dir_offset * PLAYER_ANIM_FRAME_COUNT;
        }
    }
}
