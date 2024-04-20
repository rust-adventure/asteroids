use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    assets::ImageAssets,
    kenney_assets::KenneySpriteSheetAsset,
    meteors::{MeteorBundle, MeteorType},
    ship::PlayerShipType,
    GameState, Player,
};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>()
            .add_event::<LevelCompleteEvent>()
            .add_systems(
                Update,
                (level_completion, on_level_complete)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource, Deref, DerefMut, PartialEq, Eq)]
pub struct Level(usize);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Event)]
struct LevelCompleteEvent;

fn level_completion(
    meteors: Query<Entity, With<MeteorType>>,
    next_level: Res<Level>,
    mut local_level: Local<Level>,
    mut sent_level_complete_event: Local<bool>,
    mut events: EventWriter<LevelCompleteEvent>,
) {
    if *next_level != *local_level && !meteors.is_empty() {
        local_level.0 = next_level.0;
        *sent_level_complete_event = false;
    }
    if meteors.is_empty()
        && *next_level == *local_level
        && *sent_level_complete_event == false
    {
        info!("Level Complete");
        events.send(LevelCompleteEvent);
        *sent_level_complete_event = true;
    }
}

fn on_level_complete(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut events: EventReader<LevelCompleteEvent>,
    mut current_level: ResMut<Level>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        warn!("no primary window, can't start game");
        return;
    };
    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

    if !events.is_empty() {
        events.clear();
        current_level.0 += 1;

        let width = window.resolution.width() / 2.;
        let height = window.resolution.height() / 2.;

        let mut rng = rand::thread_rng();

        // TODO: Make sure meteors don't spawn on ships
        for _ in 0..current_level.0 {
            commands.spawn(MeteorBundle::big(
                Transform::from_xyz(
                    rng.gen_range(-width..width),
                    rng.gen_range(-height..height),
                    1.,
                ),
                space_sheet,
            ));
        }
    }
}
