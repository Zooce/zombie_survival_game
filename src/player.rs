use bevy::prelude::*;
use bevy::render::camera::Camera;
pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // sprite
    let player_texture_handle = asset_server.load("Player.png");

    // basic player components
    commands
        .spawn(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            ..Default::default()
        })
        .with(Player)
        ;
}

pub fn player_movement(
    time: Res<Time>,
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_events_reader: Local<EventReader<CursorMoved>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // note: cursor position is 0,0 in the bottom left corner, while top left corner is W,H
    // note: this position needs to be translated such that the player's position = W/2,H/2
    let rotation = match cursor_moved_events_reader.latest(&cursor_moved_events) {
        Some(e) => {
            if let Some(win) = windows.get_primary() {
                let x = e.position.x - win.width() / 2.0;
                let y = e.position.y - win.height() / 2.0;
                Some(Quat::from_rotation_z(y.atan2(x)))
            } else {
                None
            }
        }
        None => None
    };

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
        let player_speed = 200.0 * time.delta_seconds();
        input_vector = input_vector.normalize() * player_speed;
    }

    // there's only one player
    if let Some(mut player_transform) = player_query.iter_mut().last() {
        player_transform.translation += input_vector;
        if let Some(r) = rotation {
            player_transform.rotation = r;
        }
    }

    // there's only one camera
    if let Some(mut camera_transform) = camera_query.iter_mut().last() {
        camera_transform.translation += input_vector;
    }
}
