use bevy::prelude::*;

pub struct ResourceHandles {
    pub player_handle: Handle<ColorMaterial>,
    pub zombie_handle: Handle<ColorMaterial>,
    pub bullet_handle: Handle<ColorMaterial>,

    pub debug_hurt_collider_handle: Handle<ColorMaterial>,
    pub debug_hit_collider_handle: Handle<ColorMaterial>,

    // pub debug_collider_radius_handle: Handle<ColorMaterial>,
    // pub debug_hit_collider_handle: Handle<ColorMaterial>,
    // pub debug_hurt_collider_handle: Handle<ColorMaterial>,

    pub gun_audio_handle: Handle<AudioSource>,

    // pub player_texture_atlas_handle: Handle<TextureAtlas>,
}

pub struct ColliderRadius(pub f32);

pub struct Health {
    pub points: i64,
}

pub struct Attack {
    pub damage: i64,
}

#[derive(Debug)]
pub struct HitCollider {
    pub radius: f32,
    pub transform: Transform, // the transform is an offset from the owner
}

#[derive(Debug)]
pub struct HurtCollider {
    pub radius: f32,
    pub offset: Vec3,
}

// just a utility function
pub fn circles_collide(r1: f32, p1: Vec3, r2: f32, p2: Vec3) -> bool {
    // two circles in 2D are colliding iff:
    //
    //      r1 + r2  >  sqrt((x2 - x1)^2 + (y2 - y1)^2)
    //
    // or even better:
    //
    //      (r1 + r2)^2  >  (x2 - x1)^2 + (y2 - y1)^2
    let max_dist = (r1+ r2).powi(2);
    let diff = p1 - p2;
    let dist = (diff.x).powi(2) + (diff.y).powi(2);
    dist < max_dist
}
