use bevy::prelude::*;

use crate::bullet::*;
use crate::common::*;

#[derive(Default)]
pub struct ShootEvent(pub Transform);

pub fn handle_shoot_events(
    shoot_events: Res<Events<ShootEvent>>,
    mut shoot_events_reader: Local<EventReader<ShootEvent>>,
    commands: &mut Commands,
    texture_handles: Res<ResourceHandles>,
) {
    if let Some(event) = shoot_events_reader.iter(&shoot_events).next_back() {
        spawn_bullet(commands, &texture_handles, event.0);
    }
}
