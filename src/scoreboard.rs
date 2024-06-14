use bevy::prelude::*;

use crate::score::Score;

#[derive(Component)]
pub struct PlayerScoreboard;

#[derive(Component)]
pub struct AiScoreboard;

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

/// Update scoreboard
/// 
/// 
pub fn update_scoreboard(
    mut player_score: Query<
        &mut Text,
        With<PlayerScoreboard>
    >,
    mut ai_score: Query<
        &mut Text,
        (With<AiScoreboard>, Without<PlayerScoreboard>)
    >,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }
        
        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.sections[0].value = score.ai.to_string();
        }
    } 
}

/// Spawn scoreboard
/// 
/// 
pub fn spawn_scoreboard(
    mut commands: Commands,
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a
        // single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts 
            // into a `String`, such as `&str`
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            }
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(15.),
            ..default()
        }),
        PlayerScore
    ));
    
    // Do it for the ai score
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            }
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(15.),
            ..default()
        }),
        AiScore,
    ));
}
