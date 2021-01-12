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
        .init_resource::<ZombieTimer>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .add_system(zombie_movement.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // draw the temp player sprite
    let background_texture_handle = asset_server.load("test-background.png");
    let player_texture_handle = asset_server.load("Player.png");
    let zombie_texture_handle = asset_server.load("Enemy.png");
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
        .with(Zombie{ angle: 0.0 })
        ;
}

struct Background;
//------------------------------------------------------------------------------ Player
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
        let player_speed = 200.0 * time.delta_seconds();
        input_vector = input_vector.normalize() * player_speed;
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

//------------------------------------------------------------------------------ Zombie

struct Zombie {
    angle: f32,
}

struct ZombieTimer {
    timer: Timer,
}

impl Default for ZombieTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, true),
        }
    }
}

use rand::{thread_rng, Rng};
fn zombie_movement(
    time: Res<Time>,
    mut zombie_timer: ResMut<ZombieTimer>,
    mut zombie_query: Query<(&mut Zombie, &mut Transform)>,
) {
    zombie_timer.timer.tick(time.delta_seconds());
    let mut rng = thread_rng();
    let zombie_speed = 50.0 * time.delta_seconds();
    for (mut zombie, mut zombie_transform) in zombie_query.iter_mut() {
        if zombie_timer.timer.finished() {
            zombie.angle += rng.gen_range(-1.0..1.0) * (std::f32::consts::PI / 2.0);
            println!("new zombie angle: {}, x: {}, y: {}", zombie.angle, zombie.angle.cos(), zombie.angle.sin());
        }
        zombie_transform.rotation = Quat::from_rotation_z(zombie.angle);
        zombie_transform.translation += Vec3::new(zombie.angle.cos(), zombie.angle.sin(), 0.0) * zombie_speed;
    }
}
