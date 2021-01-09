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
    let player_texture_handle = asset_server.load("player.png");
    let zombie_texture_handle = asset_server.load("zombie.png");
    commands
        .spawn(Camera2dBundle::default())
        // background sprite
        .spawn(SpriteBundle {
            material: materials.add(background_texture_handle.into()),
            ..Default::default()
        })
        .with(Background)
        // player sprite
        .spawn(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            ..Default::default()
        })
        .with(Player)
        // zombie sprite
        .spawn(SpriteBundle {
            material: materials.add(zombie_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(-300.0, 200.0, 0.0)),
            ..Default::default()
        })
        .with(Zombie)
        ;
}

struct Background;
struct Zombie;
struct Player;

fn player_movement(
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
        let delta = 200.0 * time.delta_seconds();
        input_vector = input_vector.normalize() * delta;
    }

    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += input_vector;
        if let Some(r) = rotation {
            player_transform.rotation = r;
        }
    }

    for mut camera_transform in camera_query.iter_mut() {
        camera_transform.translation += input_vector;
    }
}
