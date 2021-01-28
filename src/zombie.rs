use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::common::*;

pub struct Zombie {
    angle: f32,
}

pub struct ZombieTimer {
    timer: Timer,
}

impl Default for ZombieTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, true),
        }
    }
}

pub fn spawn_zombie(
    commands: &mut Commands,
    resource_handles: &ResourceHandles,
) {
    commands
        .spawn(SpriteBundle {
            material: resource_handles.zombie_handle.clone(),
            transform: Transform::from_translation(Vec3::new(-300.0, 200.0, 0.0)),
            ..Default::default()
        })
        .with(Zombie { angle: 0.0 })
        .with(ColliderRadius(32.0))
        .with(Health { points: 100 })
        // debugging
        .with_children(|parent| {
            let collider_shape = shapes::Circle {
                radius: 32.0,
                ..Default::default()
            };

            parent.spawn(ShapeBuilder::build_as(
                &collider_shape,
                resource_handles.debug_collider_handle.clone(),
                TessellationMode::Stroke(StrokeOptions::default()),
                Transform::default())
            );
        })
        ;
}

pub fn zombie_movement(
    time: Res<Time>,
    mut zombie_timer: ResMut<ZombieTimer>,
    mut zombie_query: Query<(&mut Zombie, &mut Transform)>,
) {
    use rand::{thread_rng, Rng};
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
