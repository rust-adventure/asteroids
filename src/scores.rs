use bevy::prelude::*;

use crate::{
    meteors::MeteorDestroyed, ufo::UfoDestroyed, GameState,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scores>()
            .add_systems(
                Update,
                (score_ufos, score_meteors, render_score)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_scores_ui,
            )
            .add_systems(
                OnExit(GameState::Playing),
                remove_scores_ui,
            );
    }
}

#[derive(Resource, PartialEq, Eq, Debug, Default)]
pub struct Scores {
    pub current: usize,
    pub high: usize,
}

fn score_meteors(
    mut scores: ResMut<Scores>,
    mut reader: EventReader<MeteorDestroyed>,
) {
    for meteor in reader.read() {
        let meteor_score_value = match meteor.destroyed_type
        {
            crate::meteors::MeteorType::Big => 20,
            crate::meteors::MeteorType::Medium => 60,
            crate::meteors::MeteorType::Small => 100,
        };
        scores.current += meteor_score_value;
    }
}

fn score_ufos(
    mut scores: ResMut<Scores>,
    mut reader: EventReader<UfoDestroyed>,
) {
    for _ufo in reader.read() {
        scores.current += 500;
    }
}

#[derive(Component)]
struct ScoreContainer;

#[derive(Component)]
struct ScoreDisplay;

// when we start playing
fn spawn_scores_ui(mut commands: Commands) {
    let id = commands
        .spawn((
            TextBundle {
                // background_color: Color::RED.into(),
                text: Text::from_section(
                    // Accepts a String or any type that converts into a String, such as &str.
                    "hello world!",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            },
            ScoreDisplay,
        ))
        .id();
    commands
        .spawn((
            NodeBundle {
                // background_color: Color::GREEN.into(),
                style: Style {
                    padding: UiRect::all(Val::Px(20.)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            ScoreContainer,
        ))
        .add_child(id);
}

fn remove_scores_ui(
    mut commands: Commands,
    query: Query<Entity, With<ScoreContainer>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn render_score(
    mut query: Query<&mut Text, With<ScoreDisplay>>,
    scores: Res<Scores>,
) {
    for mut text in &mut query {
        let Some(section) = text.sections.get_mut(0) else {
            error_once!("ScoreDisplay text section doesn't have a 0th section");
            continue;
        };

        section.value = scores.current.to_string();
    }
}
