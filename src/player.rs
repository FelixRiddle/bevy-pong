use bevy::prelude::*;

use crate::gutter::GUTTER_HEIGHT;
use crate::{
    Velocity,
    Position,
};
use crate::paddle::{Paddle, PADDLE_HEIGHT, PADDLE_SPEED};

// Where is player defined in the tutorial??????
// ???????????????????????//
/// Handle player input
/// 
/// 
pub fn handle_player_input(
    _keyboard_input: Res<ButtonInput<KeyCode>>,
    // mut paddle: Query<&mut Velocity, With<Player>>
) {
    // if let Ok(mut velocity) = paddle.get_single_mut() {
    //     if keyboard_input.pressed(KeyCode::ArrowUp) {
    //         velocity.0.y = 1.;
    //     } else if keyboard_input.pressed(KeyCode::ArrowDown) {
    //         velocity.0.y = -1.;
    //     } else {
    //         velocity.0.y = 0.;
    //     }
    // }
}

/// Move paddles
/// 
/// 
pub fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;
        
        for(mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}
