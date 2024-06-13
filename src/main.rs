use std::error::Error;

use bevy::prelude::*;

pub mod ball_bundle;
pub mod circle_collision;
pub mod gutter;
pub mod paddle;

use ball_bundle::move_ball;

#[derive(Component)]
pub struct Position(Vec2);

// This component is a tuple type, we can access the Vec2 it holds
// by using the position of the item in the tuple 
// e.g. velocity.0 which would be a Vec2
#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Shape(Vec2);

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
    // Give me all transforms and positions from entities 
    // that contain both
    mut positionables: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in positionables.iter_mut() {
        // Our position is `Vec2` but a translation is `Vec3`
        // so we extend our `Vec2` into one by adding a `z`
        // value of 0
        transform.translation = position.0.extend(0.);
    }
}

/// Main
/// 
/// 
fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            ball_bundle::spawn_ball,
            spawn_camera,
            paddle::spawn_paddles,
        ))
        .add_systems(Update, (
            move_ball,
            // Add our projection system to run after
            // we move our ball so we are not reading
            // movement one frame behind
            project_positions.after(move_ball),
            ball_bundle::handle_collisions.after(move_ball),
        ))
        .run();
    
    Ok(())
}
