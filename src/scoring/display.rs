use super::*;
use crate::colors;
use bevy::prelude::*;

const OFFSET_TEXT_FROM_BOARD: f32 = 15.0;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct HighScoreDisplay;

pub fn scorekeeping_ui(
    mut commands: Commands,
    score: Res<Score>,
    high_score: Res<HighScore>,
    fonts: Res<FontAssets>,
    board: Res<Board>,
) {
    let alfa_style = TextStyle {
        font: fonts.alfa_slab_one_regular.clone(),
        font_size: 25.0,
        color: colors::TEXT,
    };
    let roboto_style = TextStyle {
        font: fonts.roboto.clone(),
        font_size: 50.0,
        color: colors::TEXT,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "Current Score\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: score.score.to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " apples".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: "\nTime\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: "0".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " seconds".to_string(),
                    style: roboto_style.clone(),
                },
            ])
            .with_justify(JustifyText::Right),
            transform: Transform::from_xyz(
                board.low_edge() - OFFSET_TEXT_FROM_BOARD,
                board.high_edge(),
                1.0,
            ),
            text_anchor: Anchor::TopRight,
            ..default()
        },
        ScoreDisplay,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "High Score\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " apples".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: "\nBest Time\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: high_score
                        .time
                        .as_secs()
                        .to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " seconds".to_string(),
                    style: roboto_style.clone(),
                },
            ]),
            transform: Transform::from_xyz(
                board.high_edge() + OFFSET_TEXT_FROM_BOARD,
                board.high_edge(),
                1.0,
            ),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
        HighScoreDisplay,
    ));
}

pub fn update_score_displays(
    score: Res<Score>,
    high_score: Res<HighScore>,
    mut query_scores: Query<
        &mut Text,
        (
            With<ScoreDisplay>,
            Without<HighScoreDisplay>,
        ),
    >,
    mut query_high_scores: Query<
        &mut Text,
        (
            With<HighScoreDisplay>,
            Without<ScoreDisplay>,
        ),
    >,
    timer: ResMut<crate::scoring::Timer>,
) {
    let mut text = query_scores.single_mut();
    text.sections[1].value = score.score.to_string();

    let elapsed = timer
        .runtime
        .map(|duration| duration.as_secs())
        .or(timer
            .start
            .map(|start| start.elapsed().as_secs()))
        .unwrap_or(0);
    text.sections[4].value = elapsed.to_string();

    let mut text = query_high_scores.single_mut();
    text.sections[1].value = high_score.score.to_string();
    text.sections[4].value =
        high_score.time.as_secs().to_string();
}
