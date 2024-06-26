use bevy::prelude::*;

use crate::Velocity;
use crate::Position;
use crate::ball::Ball;

pub enum Scorer {
    Ai,
    Player
}

#[derive(Event)]
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
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        
        if let Ok(ball) = ball.get_single_mut() {
            // Here we write our events using our event writer
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

/// Update score
/// 
/// 
pub fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>,
) {
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
    
    println!("Score: {} - {}", score.player, score.ai);
}

/// Reset ball
/// 
/// 
pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((
            mut position,
            mut velocity,
        )) = ball.get_single_mut() {
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
