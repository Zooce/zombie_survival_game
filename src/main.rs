use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod common;
use crate::common::*;

mod player;
use crate::player::*;

mod zombie;
use crate::zombie::*;

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
            .with(Attack{ damage: 10 })
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
    commands: &mut Commands,
    mut zombie_query: Query<(Entity, &ColliderRadius, &Transform, &mut Health), With<Zombie>>,
    bullet_query: Query<(Entity, &ColliderRadius, &Transform, &Attack), With<Bullet>>,
) {
    // two circles in 2D are colliding iff:
    //
    //      r1 + r2  >  sqrt((x2 - x1)^2 + (y2 - y1)^2)
    //
    // or even better:
    //
    //      (r1 + r2)^2  >  (x2 - x1)^2 + (y2 - y1)^2

    for (be, br, bt, attack) in bullet_query.iter() {
        for (ze, zr, zt, mut health) in zombie_query.iter_mut() {
            let max_dist = (br.0 + zr.0).powi(2);
            let dist = (bt.translation.x - zt.translation.x).powi(2)
                + (bt.translation.y - zt.translation.y).powi(2);
            if dist < max_dist {
                // todo list:
                //  - play "got shot" sound effect
                //  - display blood splatter on the ground (this also needs to time out...?)
                //  - maybe move these different parts to different systems?
                //  - play some kind of animation on the zombie, showing that it took damage
                health.points -= attack.damage;
                if health.points <= 0 {
                    commands.despawn_recursive(ze);
                }
                commands.despawn_recursive(be);
            }
        }
    }
}
