use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::animation::*;
use crate::bullet::*;
use crate::common::*;
use crate::player::*;

pub fn check_mouse_click_events(
    mouse_button_input: Res<Input<MouseButton>>,
    bullet_spawn_info: Res<BulletSpawnInfo>,
    commands: &mut Commands,
    resource_handles: Res<ResourceHandles>,
    audio: Res<Audio>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        spawn_bullet(commands, &resource_handles, bullet_spawn_info.transform.clone());
        audio.play(resource_handles.gun_audio_handle.clone());
    }
}

pub fn check_mouse_move_events(
    mut bullet_spawn_info: ResMut<BulletSpawnInfo>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_events_reader: Local<EventReader<CursorMoved>>,
    windows: Res<Windows>,
    mut player_animation_state_query: Query<&mut AnimationState, With<Player>>,
) {
    // note: cursor position is 0,0 in the bottom left corner, while top left corner is W,H
    // note: this position needs to be translated such that the player's position = W/2,H/2
    if let Some(e) = cursor_moved_events_reader.latest(&cursor_moved_events) {
        if let Some(win) = windows.get(e.id) {
            let x = e.position.x - win.width() / 2.0;
            let y = e.position.y - win.height() / 2.0;
            let angle = y.atan2(x);

            bullet_spawn_info.transform.rotation = Quat::from_rotation_z(angle);

            // there's only one player
            if let Some(mut animation_state) = player_animation_state_query.iter_mut().last() {
                const PI_8: f32 = std::f32::consts::PI / 8.0;
                animation_state.dir_offset = if angle > 0.0 {
                    if angle < PI_8 { 6 }
                    else if angle < 3.0 * PI_8 { 5 }
                    else if angle < 5.0 * PI_8 { 4 }
                    else if angle < 7.0 * PI_8 { 3 }
                    else if angle <= std::f32::consts::PI { 2 }
                    else {
                        println!("{} > PI", angle);
                        0
                    }
                } else {
                    if angle >= -PI_8 { 6 }
                    else if angle >= -3.0 * PI_8 { 7 }
                    else if angle >= -5.0 * PI_8 { 0 }
                    else if angle >= -7.0 * PI_8 { 1 }
                    else if angle >= -std::f32::consts::PI { 2 }
                    else {
                        println!("{} < PI", angle);
                        0
                    }
                }
            }
        }
    }
}

pub fn check_keyboard_events(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut bullet_spawn_info: ResMut<BulletSpawnInfo>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {

    let mut input_vector = Vec3::zero();
    if keyboard_input.pressed(KeyCode::A) {
        input_vector.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input_vector.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        input_vector.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input_vector.y -= 1.0;
    }

    if input_vector != Vec3::zero() {
        // TODO: move this speed to the Player component
        let player_speed = 200.0 * time.delta_seconds();
        input_vector = input_vector.normalize() * player_speed;
    }

    // there's only one player
    if let Some((mut player_transform, mut anim_state)) = player_query.iter_mut().last() {
        if input_vector == Vec3::zero() {
            anim_state.is_walking = false;
        } else {
            player_transform.translation += input_vector;
            anim_state.is_walking = true;

            // we want bullets to spawn from the player's current position
            bullet_spawn_info.transform.translation = player_transform.translation;

            // there's only one camera
            if let Some(mut camera_transform) = camera_query.iter_mut().last() {
                camera_transform.translation += input_vector;
            }
        }
    }
}
