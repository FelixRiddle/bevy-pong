use bevy::prelude::*;

use crate::Position;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    pub fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(0., 0.))
        }
    }
}

/// Spawn ball
/// 
/// 
pub fn spawn_ball(
    mut commands: Commands,
) {
    commands.spawn_empty()
        .insert(Transform::default())
        .insert(BallBundle::new());
}
