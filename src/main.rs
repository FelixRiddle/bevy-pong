use std::error::Error;

use bevy::prelude::*;

pub mod ball_bundle;

#[derive(Component)]
pub struct Position(Vec2);

/// Spawn camera
/// 
/// 
pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn_empty()
        .insert(Camera2dBundle::default());
}

/// Project positions
/// 
/// 
fn project_positions(
    mut positionables: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in positionables.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}

/// Main
/// 
/// 
fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (ball_bundle::spawn_ball, spawn_camera))
        .add_systems(Update, project_positions)
        .run();
    
    Ok(())
}
