use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod bullet;
use crate::bullet::*;

mod common;
use crate::common::*;

mod events;
use crate::events::*;

mod user_controls;
use crate::user_controls::*;

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
        .add_resource(BulletSpawnInfo{ transform: Transform::default() })
        .add_event::<ShootEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_stage("insert_resources", SystemStage::single(insert_reusable_resources.system()))
        .add_startup_stage_after("insert_resources", "setup", SystemStage::single(setup.system()))
        .add_system(check_keyboard_events.system())
        .add_system(animate_player.system())
        .add_system(zombie_movement.system())
        .add_system(check_mouse_click_events.system())
        .add_system(handle_shoot_events.system())
        .add_system(bullet_movement.system())
        .add_system(check_mouse_position.system())
        .add_system(bullet_decay.system())
        .add_system(check_bullet_collisions.system())
        .run();
}

fn insert_reusable_resources(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 6, 8);
    let resource_handles = ResourceHandles {
        zombie_handle: materials.add(asset_server.load("Enemy.png").into()),
        bullet_handle: materials.add(asset_server.load("Bullet.png").into()),
        debug_collider_handle: materials.add(Color::GREEN.into()),

        gun_audio_handle: asset_server.load("382735__schots__gun-shot.mp3"),

        player_texture_atlas_handle: texture_atlases.add(texture_atlas),
    };
    commands.insert_resource(resource_handles);
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resource_handles: Res<ResourceHandles>,
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

    spawn_player(commands, &resource_handles);
    spawn_zombie(commands, &resource_handles);
}

struct Background;
