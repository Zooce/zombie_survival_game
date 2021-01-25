use bevy::prelude::*;

pub struct ResourceHandles {
    pub zombie_handle: Handle<ColorMaterial>,
    pub bullet_handle: Handle<ColorMaterial>,
    pub debug_collider_handle: Handle<ColorMaterial>,

    pub gun_audio_handle: Handle<AudioSource>,

    pub player_texture_atlas_handle: Handle<TextureAtlas>,
}

pub struct ColliderRadius(pub f32);

pub struct Health {
    pub points: i64,
}

pub struct Attack {
    pub damage: i64,
}
