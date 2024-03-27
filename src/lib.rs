use assets::{space::SpaceSheet, ImageAssets};
use bevy::math::bounding::{
    Aabb2d, BoundingCircle, BoundingVolume,
    IntersectsVolume,
};
use bevy::{prelude::*, render::primitives::Aabb};
use bevy_xpbd_2d::prelude::*;
use controls::Laser;
use itertools::Itertools;
use meteors::{
    Meteor, MeteorBundle, MeteorDestroyed, MeteorType,
};
use movement::WrappingMovement;
use ship::{PlayerShipType, SpawnFrom};

pub mod assets;
pub mod colors;
pub mod controls;
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
    // where the ship should spawn from before landing at
    // 0,0
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
        WrappingMovement,
    ));

    commands.spawn(MeteorBundle::big(
        Transform::from_xyz(50., 0., 1.),
        &images,
        space_sheet_layout.0.clone(),
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
        if colliding_entities.len() > 0 {
            for entity_laser in &lasers {
                if colliding_entities
                    .contains(&entity_laser)
                {
                    println!(
                        "Meteor {:?} was hit by laser {:?}",
                        entity_meteor, entity_laser
                    );
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
