use bevy::prelude::*;

pub struct TextureHandles {
    pub zombie_handle: Handle<ColorMaterial>,
    pub bullet_handle: Handle<ColorMaterial>,
    pub debug_collider_handle: Handle<ColorMaterial>,
}

pub struct ColliderRadius(pub f32);

pub struct Health {
    pub points: i64,
}

pub struct Attack {
    pub damage: i64,
}
