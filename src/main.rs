use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod common;
use crate::common::*;

mod bullet;
use crate::bullet::*;

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
