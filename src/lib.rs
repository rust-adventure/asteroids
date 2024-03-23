use assets::{space::SpaceSheet, ImageAssets};
use bevy::prelude::*;
use ship::{PlayerShipType, SpawnFrom};

pub mod assets;
pub mod colors;
pub mod controls;
pub mod settings;
pub mod ship;
pub mod ui;

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
    Menu,
    ChooseShip,
    Playing,
}

#[derive(Component)]
struct Player;

pub fn start_game(
    mut commands: Commands,
    images: Res<ImageAssets>,
    space_sheet_layout: Res<SpaceSheet>,
    player_ship_type: Res<PlayerShipType>,
    // where the ship should spawn from before landing at 0,0
    spawn_from: Res<SpawnFrom>,
) {
    commands.spawn((
        SpriteBundle {
            // transform: Transform::from_xyz(0., 0., 1.),
            transform: spawn_from.0,
            texture: images.space_sheet.clone(),
            ..default()
        },
        TextureAtlas {
            index: player_ship_type.base_atlas_index(),
            layout: space_sheet_layout.0.clone(),
        },
        Player,
        player_ship_type.clone(),
    ));
}
