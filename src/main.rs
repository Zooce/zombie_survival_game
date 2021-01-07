use bevy::prelude::*;
use bevy::render::camera::Camera;

fn main() {
    App::build()
    .add_resource(
        WindowDescriptor {
            title: "Zombie Survival Game".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        }
    )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // draw the temp player sprite
    let background_texture_handle = asset_server.load("test-background.png");
    let player_texture_handle = asset_server.load("icon.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(background_texture_handle.into()),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            ..Default::default()
        })
        .with(Player);
}

struct Player;

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_events_reader: Local<EventReader<CursorMoved>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // note: cursor position is 0,0 in the bottom left corner, while top left corner is W,H
    // note: this position needs to be translated such that the player's position = W/2,H/2
    let cursor_pos = match cursor_moved_events_reader.latest(&cursor_moved_events) {
        Some(e) => {
            println!("M-P -> {}", e.position);
            Some(e.position)
        }
        None => None,
    };

    let delta = 200.0 * time.delta_seconds();
    let mut horz_delta = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        horz_delta -= delta;
    }
    if keyboard_input.pressed(KeyCode::D) {
        horz_delta += delta;
    }
    let mut vert_delta = 0.0;
    if keyboard_input.pressed(KeyCode::W) {
        vert_delta += delta;
    }
    if keyboard_input.pressed(KeyCode::S) {
        vert_delta -= delta;
    }

    if delta > 0.0 {
        for mut player_transform in player_query.iter_mut() {
            player_transform.translation.x += horz_delta;
            player_transform.translation.y += vert_delta;
            println!("P-P -> {}", player_transform.translation);
            println!("P-R -> {}", player_transform.rotation);
        }

        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x += horz_delta;
            camera_transform.translation.y += vert_delta;
            println!("C-P -> {}", camera_transform.translation);
        }
    }
}
