use std::error::Error;

use bevy::prelude::*;

pub mod ai;
pub mod ball;
pub mod circle_collision;
pub mod gutter;
pub mod paddle;
pub mod player;
pub mod score;

use ball::move_ball;
use score::{
    detect_scoring,
    Score,
    Scored,
};
use player::handle_player_input;

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
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(Startup, (
            ball::spawn_ball,
            spawn_camera,
            paddle::spawn_paddles,
            gutter::spawn_gutters,
        ))
        .add_systems(Update, (
            move_ball,
            handle_player_input,
            detect_scoring,
            score::reset_ball.after(detect_scoring),
            score::update_score.after(detect_scoring),
            paddle::move_paddles.after(handle_player_input),
            // Add our projection system to run after
            // we move our ball so we are not reading
            // movement one frame behind
            project_positions.after(move_ball),
            ball::handle_collisions.after(move_ball),
        ))
        .run();
    
    Ok(())
}
