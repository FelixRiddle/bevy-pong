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
    mut player_score: Query<&mut Text, With<PlayerScoreboard>>,
    mut ai_score: Query<&mut Text, (With<AiScoreboard>, Without<PlayerScoreboard>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for mut text in &mut player_score {
            **text = score.player.to_string();
        }

        for mut text in &mut ai_score {
            **text = score.ai.to_string();
        }
    }
}

/// Spawn scoreboard
///
///
pub fn spawn_scoreboard(mut commands: Commands) {
    // Player Score (Right side)
    commands.spawn((
        Text::new("0"),
        TextFont {
            font_size: FontSize::Px(72.0),
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(15.),
            ..default()
        },
        PlayerScore,
    ));

    // AI Score (Left side)
    commands.spawn((
        Text::new("0"),
        TextFont {
            font_size: FontSize::Px(72.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(15.),
            ..default()
        },
        AiScore,
    ));
}
