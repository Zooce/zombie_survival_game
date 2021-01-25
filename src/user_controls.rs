use bevy::prelude::*;
use bevy::render::camera::Camera;

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

pub fn check_mouse_position(
    mut bullet_spawn_info: ResMut<BulletSpawnInfo>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_events_reader: Local<EventReader<CursorMoved>>,
    windows: Res<Windows>,
) {
    // note: cursor position is 0,0 in the bottom left corner, while top left corner is W,H
    // note: this position needs to be translated such that the player's position = W/2,H/2
    if let Some(e) = cursor_moved_events_reader.latest(&cursor_moved_events) {
        if let Some(win) = windows.get_primary() {
            let x = e.position.x - win.width() / 2.0;
            let y = e.position.y - win.height() / 2.0;
            bullet_spawn_info.transform.rotation = Quat::from_rotation_z(y.atan2(x));
        }
    }
}

pub fn check_keyboard_events(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut bullet_spawn_info: ResMut<BulletSpawnInfo>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
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
    if let Some(mut player_transform) = player_query.iter_mut().last() {
        player_transform.translation += input_vector;

        // we want bullets to spawn from the player's current position
        bullet_spawn_info.transform.translation = player_transform.translation;
    }

    // there's only one camera
    if let Some(mut camera_transform) = camera_query.iter_mut().last() {
        camera_transform.translation += input_vector;
    }
}
