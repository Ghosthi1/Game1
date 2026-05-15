mod map;
mod ai;
mod character;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use crate::character::CharacterPlugin;

fn main() {
    App::new()
        .insert_resource(map::map_gen::generate_map(50, 50))
        .insert_resource(map::MapOffset { offset: Vec2::new(-(50.0 * 8.0), -(50.0 * 8.0)) })
        .add_plugins((DefaultPlugins, TilemapPlugin, map::MapRendererPlugin, CharacterPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}