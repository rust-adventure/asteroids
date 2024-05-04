use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_hanabi::{EffectProperties, EffectSpawner};
use bevy_xpbd_2d::plugins::collision::Collider;
use rand::Rng;

use crate::{
    assets::ImageAssets, controls::Laser,
    kenney_assets::KenneySpriteSheetAsset,
    movement::WrappingMovement, ui::pause::Pausable,
    GameState, Player,
};

pub struct UfoPlugin;

impl Plugin for UfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (ufo_movement, ufo_weapon_system)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            FixedUpdate,
            periodically_spawn_ufo
                .run_if(in_state(GameState::Playing)),
        )
        .insert_resource(Time::<Fixed>::from_seconds(10.))
        .add_systems(
            PostUpdate,
            ufo_destroyed_event_handler
                .run_if(resource_equals(
                    Pausable::NotPaused,
                ))
                .run_if(in_state(GameState::Playing)),
        )
        .add_event::<UfoDestroyed>();
    }
}

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct UfoOwned;

fn periodically_spawn_ufo(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    window: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Ufo>,
) {
    let mut rng = rand::thread_rng();

    if !query.is_empty() || rng.gen::<f32>() < 0.2 {
        info!("did not spawn ufo");
        return;
    }

    let Ok(window) = window.get_single() else {
        warn!("no primary window, can't start game");
        return;
    };

    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

    let width = window.resolution.width() / 2.;
    let height = window.resolution.height() / 2.;

    let ufo_dimensions = space_sheet
        .textures
        .iter()
        .find(|sub_texture| {
            sub_texture.name == "ufoBlue.png"
        })
        .expect(
            "space_sheet should have a valid ufo texture",
        );
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                -500.,
                rng.gen_range(-height..height),
                1.,
            ),
            texture: space_sheet.sheet.clone(),
            ..default()
        },
        TextureAtlas {
            index: 260, // red ufo
            layout: space_sheet
                .texture_atlas_layout
                .clone(),
        },
        Collider::circle(ufo_dimensions.width as f32 / 2.),
        WrappingMovement,
        Ufo,
    ));
}

fn ufo_movement(
    mut query: Query<&mut Transform, With<Ufo>>,
    time: Res<Time>,
) {
    let preexisting_movement_factor = Vec2::new(1., 1.);
    for mut transform in &mut query {
        let ufo_facing_direction = Vec3::X;
        // transform.rotation * Vec3::Y;
        let translation_delta = preexisting_movement_factor
            + ufo_facing_direction.xy()
                * time.delta_seconds();
        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y
            * time.elapsed_seconds().sin();
    }
}

#[derive(Debug, Event)]
pub struct UfoDestroyed {
    pub destroyed_at: Transform,
}

fn ufo_destroyed_event_handler(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut events: EventReader<UfoDestroyed>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut effect: Query<(
        &mut EffectProperties,
        &mut EffectSpawner,
        &mut Transform,
    )>,
) {
    let Some(space_sheet) = sheets.get(&images.space_sheet)
    else {
        warn!("ufo_destroyed_event_handler requires ufo sprites to be loaded");
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

    for UfoDestroyed { destroyed_at } in &mut events.read()
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
    }
}

fn ufo_weapon_system(
    mut commands: Commands,
    time: Res<Time>,
    query_player: Query<&Transform, With<Player>>,
    query_ufo: Query<&Transform, With<Ufo>>,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut last_shot: Local<Option<Duration>>,
) {
    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

    let Ok(transform_player) = query_player.get_single()
    else {
        if query_player.iter().count() > 1 {
            error_once!(
                "Only expected one Player component. got {}",
                query_player.iter().count()
            );
        }
        return;
    };

    let Ok(transform_ufo) = query_ufo.get_single() else {
        if query_ufo.iter().count() > 1 {
            error_once!(
                "Only expected one Ufo component. got {}",
                query_ufo.iter().count()
            );
        }
        return;
    };

    // transform;

    // let movement_factor = (transform_player.translation
    //     - transform_ufo.translation)
    //     .xy()
    //     .normalize()
    //     * Vec2::new(1., -1.);

    let movement_factor = Vec2::ZERO;

    let can_shoot = last_shot.is_none() || {
        if let Some(shot) = *last_shot {
            time.elapsed() - shot
                > Duration::from_millis(2000)
        } else {
            false
        }
    };

    if can_shoot {
        *last_shot = Some(time.elapsed());

        commands.spawn((
            SpriteBundle {
                // transform: transform_ufo.looking_at(
                //     transform_player.translation.xyz(),
                //     Vec3::Y,
                // ),
                transform: transform_ufo.with_rotation(
                    Quat::from_rotation_z(
                        ((transform_player.translation
                            - transform_ufo.translation)
                            .yx()
                            * Vec2::new(1., -1.))
                        .to_angle(),
                    ),
                ),
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            TextureAtlas {
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
                index: 137,
            },
            Laser {
                movement_factor: movement_factor,
                speed: 300.,
            },
            UfoOwned,
            Collider::triangle(
                Vec2::new(0., -27.),
                Vec2::new(4.5, 27.),
                Vec2::new(-4.5, 27.),
            ),
        ));
    }
}
