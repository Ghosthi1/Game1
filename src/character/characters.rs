use bevy::prelude::*;
use std::collections::VecDeque;
use crate::map::Map;
use crate::ai::a_star::find_path;

/// Current tile coordinates
#[derive(Component)]
pub struct GridPosition(pub(u32, u32));

/// Path to the target
#[derive(Component)]
pub struct Path (pub VecDeque<(u32,u32)>);

/// The movement speed in tiles per second
#[derive(Component)]
pub struct Speed (pub f32);

/// Manages character spawning and movement
pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
        app.add_systems(Update, move_character);
    }
}

fn spawn_character(mut commands: Commands, map: Res<Map>) {
    let path = find_path(&map, (1,1),(10,10)).unwrap_or_default().into_iter().collect();

    commands.spawn((
        GridPosition((1,1)),
        Path(path),
        Speed(20.0),
        Sprite {
            color: Color::srgb(1.0,0.0,0.0),
            custom_size: Some(Vec2::splat(16.0)),
            ..default()
            },
        Transform::from_xyz(16.0,16.0,1.0),
));}

fn move_character (time: Res<Time>, mut query: Query<(&mut GridPosition, &mut Path, &mut Transform, &Speed)>) {
    for (mut grid_pos, mut path, mut transform, speed) in query.iter_mut() {
        let Some(next) = path.0.front() else { continue };

        let target = Vec3::new(next.0 as f32 * 16.0, next.1 as f32 * 16.0, 1.0);

        transform.translation = transform.translation.move_towards(target, speed.0 * time.delta_secs());

        if transform.translation.distance(target) < 0.1 {
            transform.translation = target;
            grid_pos.0 = *next;
            path.0.pop_front();
        }
    }
}