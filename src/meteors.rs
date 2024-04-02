use std::f32::consts::TAU;

use crate::{
    assets::ImageAssets,
    kenney_assets::KenneySpriteSheetAsset,
    movement::{LinearMovement, Spin, WrappingMovement},
    ui::pause::Pausable,
    GameState,
};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::prelude::*;

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            sandbox_meteor_destroyed_event_handler
                .run_if(resource_equals(
                    Pausable::NotPaused,
                ))
                .run_if(in_state(GameState::Playing)),
        )
        .add_event::<MeteorDestroyed>();
    }
}

#[derive(Bundle)]
pub struct MeteorBundle {
    meteor_type: MeteorType,
    meteor: Meteor,
    collider: Collider,
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    linear_movement: LinearMovement,
    spin: Spin,
    wrapping: WrappingMovement,
}
#[derive(Component, Clone, Copy)]
pub enum MeteorType {
    Big,
    Medium,
    Small,
}
#[derive(Component)]
pub struct Meteor;

const METEOR_BASE_SPEED_BIG: f32 = 1.;
const METEOR_BASE_SPEED_MEDIUM: f32 = 1.2;
const METEOR_BASE_SPEED_SMALL: f32 = 1.4;

impl MeteorBundle {
    pub fn big(
        transform: Transform,
        space_sheet: &KenneySpriteSheetAsset,
    ) -> MeteorBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * METEOR_BASE_SPEED_BIG;
        let y = rng.gen::<f32>() * METEOR_BASE_SPEED_BIG;
        let rotation = rng.gen::<f32>() * TAU;

        MeteorBundle {
            meteor_type: MeteorType::Big,
            meteor: Meteor,
            collider: Collider::circle(42.),
            sprite_bundle: SpriteBundle {
                transform,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: 163,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            linear_movement: LinearMovement {
                movement_factor: Vec2::new(
                    x as f32, y as f32,
                ),
                movement_direction: Quat::from_rotation_z(
                    rotation,
                ),
            },
            spin: Spin(1.3),
            wrapping: WrappingMovement,
        }
    }
    pub fn medium(
        transform: Transform,
        space_sheet: &KenneySpriteSheetAsset,
    ) -> MeteorBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * METEOR_BASE_SPEED_MEDIUM;
        let y = rng.gen::<f32>() * METEOR_BASE_SPEED_MEDIUM;
        let rotation = rng.gen::<f32>() * TAU;

        MeteorBundle {
            meteor_type: MeteorType::Medium,
            meteor: Meteor,
            collider: Collider::circle(21.),
            sprite_bundle: SpriteBundle {
                transform,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: 167,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            linear_movement: LinearMovement {
                movement_factor: Vec2::new(
                    x as f32, y as f32,
                ),
                movement_direction: Quat::from_rotation_z(
                    rotation,
                ),
            },
            spin: Spin(1.6),
            wrapping: WrappingMovement,
        }
    }
    pub fn small(
        transform: Transform,
        space_sheet: &KenneySpriteSheetAsset,
    ) -> MeteorBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * METEOR_BASE_SPEED_SMALL;
        let y = rng.gen::<f32>() * METEOR_BASE_SPEED_SMALL;
        let rotation = rng.gen::<f32>() * TAU;

        MeteorBundle {
            meteor_type: MeteorType::Small,
            meteor: Meteor,
            collider: Collider::circle(14.),
            sprite_bundle: SpriteBundle {
                transform,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: 169,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            linear_movement: LinearMovement {
                movement_factor: Vec2::new(
                    x as f32, y as f32,
                ),
                movement_direction: Quat::from_rotation_z(
                    rotation,
                ),
            },
            spin: Spin(2.),
            wrapping: WrappingMovement,
        }
    }
}

#[derive(Event)]
pub struct MeteorDestroyed {
    pub destroyed_at: Transform,
    pub destroyed_type: MeteorType,
}

fn sandbox_meteor_destroyed_event_handler(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut events: EventReader<MeteorDestroyed>,
    windows: Query<&Window>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
) {
    let Ok(window) = windows.get_single() else {
        warn!("sandbox_meteor_destroyed_event_handler requires a window to spawn, but no window was found (or multiple were found)");
        return;
    };
    let width = window.resolution.width();
    let height = window.resolution.height();

    let Some(space_sheet) = sheets.get(&images.space_sheet)
    else {
        warn!("sandbox_meteor_destroyed_event_handler requires meteor sprites to be loaded");
        return;
    };

    for MeteorDestroyed {
        destroyed_at,
        destroyed_type,
    } in &mut events.read()
    {
        match destroyed_type {
            MeteorType::Big => {
                // become two medium
                for _ in 0..2 {
                    let mut rng = rand::thread_rng();
                    let x: i32 = rng.gen_range(-5..5);
                    let y: i32 = rng.gen_range(-5..5);
                    commands.spawn(MeteorBundle::medium(
                        Transform::from_xyz(
                            destroyed_at.translation.x
                                + x as f32,
                            destroyed_at.translation.y
                                + y as f32,
                            1.,
                        ),
                        &space_sheet,
                    ));
                }
            }
            MeteorType::Medium => {
                // become two smol
                for _ in 0..2 {
                    let mut rng = rand::thread_rng();
                    let x: i32 = rng.gen_range(-5..5);
                    let y: i32 = rng.gen_range(-5..5);
                    commands.spawn(MeteorBundle::small(
                        Transform::from_xyz(
                            destroyed_at.translation.x
                                + x as f32,
                            destroyed_at.translation.y
                                + y as f32,
                            1.,
                        ),
                        &space_sheet,
                    ));
                }
            }
            MeteorType::Small => {
                // do nothing
                let mut rng = rand::thread_rng();
                let x: i32 = rng.gen_range(
                    (-width as i32 / 2)..(width as i32 / 2),
                );
                let y: i32 = rng.gen_range(
                    (-height as i32 / 2)
                        ..(height as i32 / 2),
                );
                commands.spawn(MeteorBundle::big(
                    Transform::from_xyz(
                        x as f32, y as f32, 1.,
                    ),
                    &space_sheet,
                ));
            }
        }
    }
}
