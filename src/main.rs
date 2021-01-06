use bevy::prelude::*;

fn main() {
    App::build()
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
    let texture_handle = asset_server.load("icon.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

fn player_movement(
    _time: Res<Time>,
    _keyboard_input: Res<Input<KeyCode>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_events_reader: Local<EventReader<CursorMoved>>,
) {
    // note: cursor position is 0,0 in the bottom left corner, while top left corner is W,H
    if let Some(e) = cursor_moved_events_reader.latest(&cursor_moved_events) {
        println!("cursor pos: {}", e.position);
    }
}
