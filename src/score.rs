use bevy::prelude::*;

use crate::ball::Ball;
use crate::Position;
use crate::Velocity;

pub enum Scorer {
    Ai,
    Player,
}

#[derive(Message)]
pub struct Scored(Scorer);

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

/// Detect scoring
///
///
pub fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: MessageWriter<Scored>,
) {
    if let Ok(window) = window.single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.single_mut() {
            // Here we write our events using our event writer
            if ball.0.x > window_width / 2. {
                events.write(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.write(Scored(Scorer::Player));
            }
        }
    }
}

/// Update score
///
///
pub fn update_score(mut score: ResMut<Score>, mut events: MessageReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => {
                score.ai += 1;
            }
            Scorer::Player => {
                score.player += 1;
            }
        }
    }

    // println!("Score: {} - {}", score.player, score.ai);
}

/// Reset ball
///
///
pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: MessageReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.single_mut() {
            match event.0 {
                Scorer::Ai => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-1., 1.);
                }
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(1., 1.);
                }
            }
        }
    }
}
