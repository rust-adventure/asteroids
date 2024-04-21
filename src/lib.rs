use assets::ImageAssets;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use controls::Laser;
use kenney_assets::KenneySpriteSheetAsset;
use levels::Level;
use lives::Lives;
use meteors::{
    Meteor, MeteorBundle, MeteorDestroyed, MeteorType,
};
use movement::WrappingMovement;
use rand::Rng;
use ship::{
    PlayerEngineFire, PlayerShipType, ShipBundle,
    ShipDestroyed,
};
use ui::choose_ship::ChooseShipEvent;

pub mod assets;
pub mod colors;
pub mod controls;
pub mod kenney_assets;
pub mod levels;
pub mod lives;
pub mod meteors;
pub mod movement;
pub mod settings;
pub mod ship;
pub mod ui;

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    ChooseShip,
    Playing,
}

#[derive(Component)]
pub struct Player;

pub fn reset_game(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    meteors: Query<Entity, With<MeteorType>>,
    mut level: ResMut<Level>,
) {
    lives.0 = 3;
    *level = Level::default();
    // reset lives count
    for entity in &meteors {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn start_game(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut player_ship_type_choice: ResMut<PlayerShipType>,
    // player_ship_type: Res<PlayerShipType>,
    // where the ship should spawn from before landing at
    // 0,0
    // spawn_from: Res<SpawnFrom>,
    mut choose_ship_reader: EventReader<
        ui::choose_ship::ChooseShipEvent,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        warn!("no primary window, can't start game");
        return;
    };

    let Some(ChooseShipEvent {
        ship_type,
        ship_menu_location,
    }) = choose_ship_reader.read().next()
    else {
        warn!("No ChooseShipEvent coming from the menu; Check to make sure events are receivable.");
        return;
    };
    *player_ship_type_choice = ship_type.clone();

    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

    let engine_fire = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    0., -60., 1.,
                ),
                texture: space_sheet.sheet.clone(),
                sprite: Sprite {
                    flip_y: true,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            TextureAtlas {
                index: 74,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            PlayerEngineFire,
        ))
        .id();
    commands
        .spawn(ShipBundle {
            sprite_bundle: SpriteBundle {
                // transform: Transform::from_xyz(0., 0.,
                // 1.),
                transform: *ship_menu_location,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: ship_type.base_atlas_index(),
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            player: Player,
            ship_type: ship_type.clone(),
            collider: ship_type.collider(),
            wrapping_movement: WrappingMovement,
        })
        .add_child(engine_fire);

    let width = window.resolution.width() / 2.;
    let height = window.resolution.height() / 2.;

    let mut rng = rand::thread_rng();
    // TODO: spawn meteors according to current Level
    // TODO: Make sure meteors don't spawn on ships
    commands.spawn(MeteorBundle::big(
        Transform::from_xyz(
            rng.gen_range(-width..width),
            rng.gen_range(-height..height),
            1.,
        ),
        space_sheet,
    ));
}

pub fn laser_meteor_collision(
    mut commands: Commands,
    mut meteor_destroyed: EventWriter<MeteorDestroyed>,
    lasers: Query<Entity, With<Laser>>,
    meteors: Query<
        (
            Entity,
            &CollidingEntities,
            &MeteorType,
            &Transform,
        ),
        With<Meteor>,
    >,
) {
    for (
        entity_meteor,
        colliding_entities,
        meteor_type,
        transform,
    ) in &meteors
    {
        if !colliding_entities.is_empty() {
            for entity_laser in &lasers {
                if colliding_entities
                    .contains(&entity_laser)
                {
                    commands
                        .entity(entity_laser)
                        .despawn_recursive();
                    commands
                        .entity(entity_meteor)
                        .despawn_recursive();

                    meteor_destroyed.send(
                        MeteorDestroyed {
                            destroyed_at: *transform,
                            destroyed_type: *meteor_type,
                        },
                    );
                }
            }
        }
    }
}

pub fn ship_meteor_collision(
    mut commands: Commands,
    mut ship_destroyed: EventWriter<ShipDestroyed>,
    meteors: Query<Entity, With<Meteor>>,
    player_ship: Query<
        (
            Entity,
            &CollidingEntities,
            &Transform,
            &PlayerShipType,
        ),
        With<Player>,
    >,
) {
    for (
        entity_player,
        colliding_entities,
        transform,
        ship_type,
    ) in &player_ship
    {
        if !colliding_entities.is_empty() {
            for entity_meteor in &meteors {
                if colliding_entities
                    .contains(&entity_meteor)
                {
                    commands
                        .entity(entity_player)
                        .despawn_recursive();

                    ship_destroyed.send(ShipDestroyed {
                        destroyed_at: *transform,
                        ship_type: ship_type.clone(),
                    });
                }
            }
        }
    }
}
