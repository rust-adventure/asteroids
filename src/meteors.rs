use std::f32::consts::TAU;

use crate::{
    assets::ImageAssets,
    kenney_assets::KenneySpriteSheetAsset,
    movement::{LinearMovement, Spin, WrappingMovement},
    ui::pause::Pausable,
    GameState,
};
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::prelude::*;

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_meteor_effect)
            .add_systems(
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

fn register_meteor_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let spawner = Spawner::once(100.0.into(), false);

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age =
        SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.5).expr();
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        lifetime,
    );

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let color = writer.prop("spawn_color").expr();
    let init_color =
        SetAttributeModifier::new(Attribute::COLOR, color);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::Y).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(TAU).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: (writer.lit(200.)
            * writer.rand(ScalarType::Float))
        .expr(),
    };

    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .with_name("explosion")
            .with_property(
                "spawn_color",
                0xFFFFFFFFu32.into(),
            )
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_color)
            .update(update_drag)
            .render(SetSizeModifier {
                size: Vec2::splat(3.).into(),
                screen_space_size: true,
            }),
    );

    commands
        .spawn((
            ParticleEffectBundle::new(effect)
                .with_spawner(spawner),
            EffectProperties::default(),
        ))
        .insert(Name::new("effect:meteor_explosion"));
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
#[derive(Debug, Component, Clone, Copy)]
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

#[derive(Debug, Event)]
pub struct MeteorDestroyed {
    pub destroyed_at: Transform,
    pub destroyed_type: MeteorType,
}

fn sandbox_meteor_destroyed_event_handler(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut events: EventReader<MeteorDestroyed>,
    // meteors: Query<Entity, With<MeteorType>>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut effect: Query<(
        &mut EffectProperties,
        &mut EffectSpawner,
        &mut Transform,
    )>,
) {
    let Some(space_sheet) = sheets.get(&images.space_sheet)
    else {
        warn!("sandbox_meteor_destroyed_event_handler requires meteor sprites to be loaded");
        return;
    };

    let mut rng = rand::thread_rng();
    // Note: On first frame where the effect spawns,
    // EffectSpawner is spawned during PostUpdate,
    // so will not be available yet. Ignore for a
    // frame if so.
    let Ok((
        mut properties,
        mut spawner,
        mut effect_transform,
    )) = effect.get_single_mut()
    else {
        warn!("effect not ready yet, returning");
        return;
    };

    for MeteorDestroyed {
        destroyed_at,
        destroyed_type,
    } in &mut events.read()
    {
        effect_transform.translation =
            destroyed_at.translation;

        let color = Color::lch(
            1.,
            1.,
            rand::random::<f32>() * 360.,
        );
        properties.set(
            "spawn_color",
            color.as_linear_rgba_u32().into(),
        );

        // Spawn the particles
        spawner.reset();

        match destroyed_type {
            MeteorType::Big => {
                // become two medium
                for _ in 0..2 {
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
                        space_sheet,
                    ));
                }
            }
            MeteorType::Medium => {
                // become two smol
                for _ in 0..2 {
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
                        space_sheet,
                    ));
                }
            }
            MeteorType::Small => {
                // small meteors don't propogate
                // more meteors
            }
        }
    }
}
