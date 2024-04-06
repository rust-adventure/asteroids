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
    // Set `spawn_immediately` to false to spawn on command with Spawner::reset()
    let spawner = Spawner::once(100.0.into(), false);

    let writer = ExprWriter::new();

    // Init the age of particles to 0, and their lifetime to 1.5 second.
    let age = writer.lit(0.).expr();
    let init_age =
        SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.5).expr();
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        lifetime,
    );

    // Add a bit of linear drag to slow down particles after the inital spawning.
    // This keeps the particle around the spawn point, making it easier to visualize
    // the different groups of particles.
    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    // Bind the initial particle color to the value of the 'spawn_color' property
    // when the particle spawns. The particle will keep that color afterward,
    // even if the property changes, because the color will be saved
    // per-particle (due to the Attribute::COLOR).
    let color = writer.prop("spawn_color").expr();
    let init_color =
        SetAttributeModifier::new(Attribute::COLOR, color);

    let normal = writer.prop("normal");

    // Set the position to be the collision point, which in this example is always
    // the emitter position (0,0,0) at the ball center, minus the ball radius
    // alongside the collision normal. Also raise particle to Z=0.2 so they appear
    // above the black background box.
    //   pos = -normal * BALL_RADIUS + Z * 0.2;
    let pos = normal.clone() * writer.lit(-20.)
        + writer.lit(Vec3::Z * 0.2);
    let init_pos = SetAttributeModifier::new(
        Attribute::POSITION,
        pos.expr(),
    );

    // Set the velocity to be a random direction mostly along the collision normal,
    // but with some spread. This cheaply ensures that we spawn only particles
    // inside the black background box (or almost; we ignore the edge case around
    // the corners). An alternative would be to use something
    // like a KillAabbModifier, but that would spawn particles and kill them
    // immediately, wasting compute resources and GPU memory.
    //   tangent = cross(Z, normal);
    //   spread = frand() * 2. - 1.;  // in [-1:1]
    //   speed = frand() * 0.2;
    //   velocity = normalize(normal + tangent * spread * 5.) * speed;
    let tangent = writer.lit(Vec3::Z).cross(normal.clone());
    let spread = writer.rand(ScalarType::Float)
        * writer.lit(20.)
        - writer.lit(1.);
    let speed =
        writer.rand(ScalarType::Float) * writer.lit(20.2);
    let velocity = (normal
        + tangent * spread * writer.lit(5.0))
    .normalized()
        * speed;
    let init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        velocity.expr(),
    );

    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .with_name("spawn_on_command")
            .with_property(
                "spawn_color",
                0xFFFFFFFFu32.into(),
            )
            .with_property("normal", Vec3::ZERO.into())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_color)
            .update(update_drag)
            // Set a size of 3 (logical) pixels, constant in screen space, independent of projection
            .render(SetSizeModifier {
                size: Vec2::splat(3.).into(),
                screen_space_size: false,
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
    mut effect: Query<
        (
            &mut EffectProperties,
            &mut EffectSpawner,
            &mut Transform,
        ),
        // Without<Ball>,
    >,
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

    let mut rng = rand::thread_rng();
    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // PostUpdate, so will not be available yet. Ignore for a frame if so.
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

        // Pick a random particle color
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        let color = 0xFF000000u32
            | (b as u32) << 16
            | (g as u32) << 8
            | (r as u32);
        properties.set("spawn_color", color.into());

        // Set the collision normal
        let normal = Vec2::Y.normalize();
        info!("Collision: n={:?}", normal);
        properties.set("normal", normal.extend(0.).into());
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
                // do nothing
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
                    space_sheet,
                ));
            }
        }
    }
}
