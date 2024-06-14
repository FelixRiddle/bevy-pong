use bevy::prelude::*;

use crate::{
    Position,
    Velocity,
};
use crate::ball::Ball;

#[derive(Component)]
pub struct Ai;

/// Move ai
/// 
/// 
pub fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((
        mut velocity,
        position
    )) = ai.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum();
        }
    }
}
