use bevy::prelude::*;

use crate::{
    assets::ImageAssets,
    kenney_assets::KenneySpriteSheetAsset,
    meteors::{MeteorBundle, MeteorDestroyed, MeteorType},
    ship::PlayerShipType,
    GameState, Player,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scores>().add_systems(
            Update,
            score_meteors
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Resource, PartialEq, Eq, Debug, Default)]
pub struct Scores {
    current: usize,
    high: usize,
}

fn score_meteors(
    mut scores: ResMut<Scores>,
    mut reader: EventReader<MeteorDestroyed>,
) {
    for meteor in reader.read() {
        dbg!(meteor);
        scores.current += 100;
        info!("{:?}", scores);
    }
    // scores.current += reader.read().len() * 100;
    // info!("{:?}", scores);
}
