use bevy::prelude::*;
use bevy::render::camera::Camera;

use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_resource(
            WindowDescriptor {
                title: "Zombie Survival Game".to_string(),
                resizable: false,
                ..Default::default()
            }
        )
        .init_resource::<ZombieTimer>()
        .add_event::<ShootEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_stage("insert_resources", SystemStage::single(insert_reusable_resources.system()))
        .add_startup_stage_after("insert_resources", "setup", SystemStage::single(setup.system()))
        .add_system(player_movement.system())
        .add_system(zombie_movement.system())
        .add_system(check_mouse_events.system())
        .add_system(handle_shoot_events.system())
        .add_system(bullet_movement.system())
        .add_system(bullet_decay.system())
        .add_system(check_bullet_collisions.system())
        .run();
}

fn insert_reusable_resources(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handles = TextureHandles {
        zombie_handle: materials.add(asset_server.load("Enemy.png").into()),
        bullet_handle: materials.add(asset_server.load("Bullet.png").into()),
        debug_collider_handle: materials.add(Color::GREEN.into()),
    };
    commands.insert_resource(texture_handles);
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_handles: Res<TextureHandles>,
) {
    // draw the temp player sprite
    let background_texture_handle = asset_server.load("test-background.png");

    commands
        .spawn(Camera2dBundle::default())

        // background sprite
        .spawn(SpriteBundle {
            material: materials.add(background_texture_handle.into()),
            ..Default::default()
        })
        .with(Background)
        ;

    spawn_player(commands, asset_server, materials);
    spawn_zombie(commands, texture_handles);
}

struct Background;

struct TextureHandles {
    zombie_handle: Handle<ColorMaterial>,
    bullet_handle: Handle<ColorMaterial>,
    debug_collider_handle: Handle<ColorMaterial>,
}

//------------------------------------------------------------------------------ Common

struct ColliderRadius(f32);


//------------------------------------------------------------------------------ Player
struct Player;

fn spawn_player(
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
            timer: Timer::from_seconds(3.0, true),
        }
    }
}

fn spawn_zombie(
    commands: &mut Commands,
    texture_handles: Res<TextureHandles>,
) {
    // debugging
    let collider_shape = shapes::Circle {
        radius: 32.0,
        ..Default::default()
    };

    commands
        .spawn(SpriteBundle {
            material: texture_handles.zombie_handle.clone(),
            transform: Transform::from_translation(Vec3::new(-300.0, 200.0, 0.0)),
            ..Default::default()
        })
        .with(Zombie { angle: 0.0 })
        .with(ColliderRadius(32.0))
        // debugging
        .with_children(|parent| {
            parent.spawn(ShapeBuilder::build_as(
                &collider_shape,
                texture_handles.debug_collider_handle.clone(),
                TessellationMode::Stroke(StrokeOptions::default()),
                Transform::default())
            );
        })
        ;
}

use rand::{thread_rng, Rng};
fn zombie_movement(
    time: Res<Time>,
    mut zombie_timer: ResMut<ZombieTimer>,
    mut zombie_query: Query<(&mut Zombie, &mut Transform)>,
) {
    zombie_timer.timer.tick(time.delta_seconds());
    let mut rng = thread_rng();
    // TODO: store this speed with the zombie? different zombies will have
    //       different speeds?
    let zombie_speed = 50.0 * time.delta_seconds();
    for (mut zombie, mut zombie_transform) in zombie_query.iter_mut() {
        if zombie_timer.timer.finished() {
            zombie.angle = rng.gen_range(-1.0..1.0) * (std::f32::consts::PI);
        }
        let towards = Quat::from_rotation_z(zombie.angle);
        zombie_transform.rotation = zombie_transform.rotation.lerp(towards, zombie_timer.timer.percent());
        zombie_transform.translation += Vec3::new(zombie.angle.cos(), zombie.angle.sin(), 0.0) * zombie_speed;
    }
}

//------------------------------------------------------------------------------ Bullets

#[derive(Default)]
struct ShootEvent(Transform);

fn check_mouse_events(
    mouse_button_input: Res<Input<MouseButton>>,
    mut shoot_events: ResMut<Events<ShootEvent>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // there's only one player
        if let Some(transform) = player_query.iter().last() {
            shoot_events.send(ShootEvent(transform.clone()));
        }
    }
}

struct Bullet {
    decay_timer: Timer,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            decay_timer: Timer::from_seconds(0.25, false),
        }
    }
}

fn handle_shoot_events(
    shoot_events: Res<Events<ShootEvent>>,
    mut shoot_events_reader: Local<EventReader<ShootEvent>>,
    commands: &mut Commands,
    texture_handles: Res<TextureHandles>,
) {
    if let Some(event) = shoot_events_reader.iter(&shoot_events).next_back() {
        // debugging
        let collider_shape = shapes::Circle {
            radius: 8.0,
            ..Default::default()
        };

        commands
            .spawn(SpriteBundle {
                material: texture_handles.bullet_handle.clone(),
                transform: event.0.clone(),
                ..Default::default()
            })
            .with(Bullet::default())
            .with(ColliderRadius(8.0))
            // debugging
            .with_children(|parent| {
                parent.spawn(ShapeBuilder::build_as(
                    &collider_shape,
                    texture_handles.debug_collider_handle.clone(),
                    TessellationMode::Stroke(StrokeOptions::default()),
                    Transform::default())
                );
            })
            ;
    }
}

fn bullet_movement(
    time: Res<Time>,
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
) {
    // TODO: store this speed with either the bullet or the weapon (it will be
    //       different for each type of weapon and/or bullet)?
    let speed = 1500.0 * time.delta_seconds();

    for mut bullet_transform in bullet_query.iter_mut() {
        let (axis, mut angle) = bullet_transform.rotation.to_axis_angle();
        angle *= axis.z;
        bullet_transform.translation += Vec3::new(angle.cos(), angle.sin(), 0.0) * speed;
    }
}

fn bullet_decay(
    commands: &mut Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in bullet_query.iter_mut() {
        if bullet.decay_timer.tick(time.delta_seconds()).finished() {
            //commands.despawn(entity);
            commands.despawn_recursive(entity);
        }
    }
}

fn check_bullet_collisions(
    zombie_query: Query<(&ColliderRadius, &Transform), With<Zombie>>,
    bullet_query: Query<(&ColliderRadius, &Transform), With<Bullet>>,
) {
    // two circles in 2D are colliding iff:
    //
    //      r1 + r2  >  sqrt((x2 - x1)^2 + (y2 - y1)^2)
    //
    // or even better:
    //
    //      (r1 + r2)^2  >  (x2 - x1)^2 + (y2 - y1)^2

    for (br, bt) in bullet_query.iter() {
        for (zr, zt) in zombie_query.iter() {
            let max_dist = (br.0 + zr.0).powi(2);
            let dist = (bt.translation.x - zt.translation.x).powi(2)
                + (bt.translation.y - zt.translation.y).powi(2);
            if dist < max_dist {
                println!("Collision {} {}", bt.translation, zt.translation);
            }
        }
    }
}
