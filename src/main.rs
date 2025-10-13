use std::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::*,
    dev_tools::states::log_transitions, prelude::*,
    render::camera::ScalingMode, window::PrimaryWindow,
};

const CANVAS_SIZE: Vec2 = Vec2::new(80., 10.);

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SLATE_950)))
        .insert_resource(Gravity::ZERO)
        .init_resource::<Lives>()
        .init_resource::<CurrentLevel>()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
        ))
        .init_state::<AppState>()
        .add_systems(Startup, startup)
        .add_systems(
            OnEnter(AppState::GameOver),
            spawn_game_over,
        )
        .add_systems(OnEnter(AppState::Playing), start_game)
        .add_systems(
            FixedUpdate,
            (
                linear_movement,
                wrapping_movement,
                rotational_movement,
                fire,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(Update, log_transitions::<AppState>)
        .add_observer(spawn_level_asteroids)
        .add_observer(spawn_ship)
        .add_systems(
            Update,
            check_end_level
                .run_if(in_state(AppState::Playing)),
        )
        .run()
}

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
#[states(scoped_entities)]
enum AppState {
    #[default]
    GameOver,
    Playing,
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: CANVAS_SIZE.x,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

#[derive(Resource)]
struct CurrentLevel(u32);

impl Default for CurrentLevel {
    fn default() -> Self {
        Self(1)
    }
}

fn check_end_level(
    query: Query<&Asteroid>,
    mut spawn_level: ResMut<CurrentLevel>,
    mut commands: Commands,
) {
    if query.is_empty() {
        spawn_level.0 += 1;
        commands.trigger(SpawnLevel);
    }
}

fn spawn_game_over(mut commands: Commands) {
    commands
        .spawn((
            StateScoped(AppState::GameOver),
            Button,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![Text::new("New Game")],
        ))
        .observe(
            |_trigger: Trigger<Pointer<Click>>,
             mut next_state: ResMut<
                NextState<AppState>,
            >| {
                next_state.set(AppState::Playing);
            },
        );
}

#[derive(Event)]
struct SpawnShip;

fn spawn_ship(
    _trigger: Trigger<SpawnShip>,
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let mut gizmo = GizmoAsset::default();

    let ship = Triangle2d::new(
        Vec2::new(-0.6, -1.),
        Vec2::new(0., 1.),
        Vec2::new(0.6, -1.),
    );
    gizmo.primitive_2d(
        &ship,
        Isometry2d::default(),
        SLATE_50,
    );

    let mut propulsion_gizmo = GizmoAsset::default();

    propulsion_gizmo.primitive_2d(
        &Triangle2d::new(
            Vec2::new(-0.6, -1.),
            Vec2::new(0., -1.2),
            Vec2::new(0.6, -1.),
        ),
        Isometry2d::default(),
        RED_400,
    );

    commands
        .spawn((
            Ship,
            Gizmo {
                handle: gizmo_assets.add(gizmo),
                ..default()
            },
            StateScoped(AppState::GameOver),
            RigidBody::Dynamic,
            Collider::triangle(
                ship.vertices[0],
                ship.vertices[1],
                ship.vertices[2],
            ),
            LinearVelocity::ZERO,
            ExternalForce::ZERO.with_persistence(false),
            WrappingMovement,
            RotationSpeed(Rot2::degrees(180.)),
            children![(
                Propulsion(Gizmo {
                    handle: gizmo_assets
                        .add(propulsion_gizmo.clone()),
                    ..default()
                }),
                Gizmo {
                    handle: gizmo_assets
                        .add(propulsion_gizmo),
                    ..default()
                },
            )],
            CollisionLayers::new(
                GameLayer::Ship,
                [GameLayer::Asteroid],
            ),
            CollisionEventsEnabled,
        ))
        .observe(
            |trigger: Trigger<OnCollisionStart>,
             mut lives: ResMut<Lives>,
             mut next_state: ResMut<
                NextState<AppState>,
            >,
             mut commands: Commands| {
                let Some(n) = lives.0.checked_sub(1) else {
                    next_state.set(AppState::GameOver);
                    return;
                };
                lives.0 = n;
                commands.entity(trigger.target()).despawn();
                commands.trigger(SpawnShip);
            },
        );
}

#[derive(Component)]
struct Propulsion(Gizmo);

#[derive(Component)]
struct Ship;

#[derive(Resource)]
struct Lives(u32);

impl Default for Lives {
    fn default() -> Self {
        Self(3)
    }
}

fn start_game(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut current_level: ResMut<CurrentLevel>,
) {
    *lives = Default::default();
    *current_level = Default::default();

    commands.trigger(SpawnShip);
    commands.trigger(SpawnLevel);
}

#[derive(Component, Default)]
pub struct WrappingMovement;

fn wrapping_movement(
    mut wrappers: Query<
        &mut Transform,
        With<WrappingMovement>,
    >,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let (camera, camera_transform) = camera.into_inner();

    let bottom_right = camera.viewport_to_world_2d(
        camera_transform,
        window.resolution.size(),
    )?;
    let top_left = camera.viewport_to_world_2d(
        camera_transform,
        Vec2::ZERO,
    )?;

    let width_height = (bottom_right - top_left).abs();

    for mut transform in &mut wrappers {
        if transform.translation.x > bottom_right.x {
            transform.translation.x -= width_height.x;
        } else if transform.translation.x < top_left.x {
            transform.translation.x += width_height.x;
        }

        if transform.translation.y < bottom_right.y {
            transform.translation.y += width_height.y;
        } else if transform.translation.y > top_left.y {
            transform.translation.y -= width_height.y;
        }
    }
    Ok(())
}

fn linear_movement(
    ship: Single<
        (&Transform, &mut ExternalForce),
        With<Ship>,
    >,
    input: Res<ButtonInput<KeyCode>>,
    propulsion: Single<(Entity, &Propulsion, Has<Gizmo>)>,
    mut commands: Commands,
) {
    let (entity, propulsion, has_gizmo) =
        propulsion.into_inner();

    if has_gizmo {
        commands.entity(entity).remove::<Gizmo>();
    }

    let (transform, mut force) = ship.into_inner();

    if input.pressed(KeyCode::KeyW) {
        if !has_gizmo {
            commands
                .entity(entity)
                .insert(propulsion.0.clone());
        }

        force.apply_force(
            (transform.rotation * Vec3::Y * 8.).xy(),
        );
    }
}

#[derive(Component)]
pub struct RotationSpeed(pub Rot2);

fn rotational_movement(
    mut spinnable: Query<(&RotationSpeed, &mut Transform)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (rotational_speed, mut transform) in &mut spinnable
    {
        if input.pressed(KeyCode::KeyA) {
            transform.rotate_z(
                -rotational_speed.0.as_radians()
                    * time.delta_secs(),
            );
        } else {
            if input.pressed(KeyCode::KeyD) {
                transform.rotate_z(
                    rotational_speed.0.as_radians()
                        * time.delta_secs(),
                );
            }
        }
    }
}

#[derive(Event)]
struct SpawnLevel;

#[derive(Clone)]
enum AsteroidSize {
    Small,
    Medium,
    Large,
}

impl AsteroidSize {
    fn radius(&self) -> f32 {
        match self {
            AsteroidSize::Small => 1.,
            AsteroidSize::Medium => 2.,
            AsteroidSize::Large => 3.,
        }
    }
}

#[derive(Component, Clone)]
#[require(
    RigidBody::Kinematic,
    WrappingMovement,
    CollisionLayers::new(
        GameLayer::Asteroid,
        [GameLayer::Ship],
    ),
    CollisionEventsEnabled,
    LinearVelocity,
    Transform,
    StateScoped::<AppState>(AppState::GameOver)
)]
struct Asteroid {
    size: AsteroidSize,
}

impl Asteroid {
    fn to_fragments(&self) -> u32 {
        match self.size {
            AsteroidSize::Small => 0,
            AsteroidSize::Medium => 2,
            AsteroidSize::Large => 4,
        }
    }
}

fn spawn_level_asteroids(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    current_level: Res<CurrentLevel>,
) -> Result {
    let (camera, camera_transform) = camera.into_inner();

    let bottom_right = camera.viewport_to_world_2d(
        camera_transform,
        window.resolution.size(),
    )?;
    let top_left = camera.viewport_to_world_2d(
        camera_transform,
        Vec2::ZERO,
    )?;

    for _ in 0..current_level.0 {
        use rand::{Rng, thread_rng};

        let mut rng = thread_rng();
        let velocity = rng.r#gen::<f32>() * 10.;
        let angle =
            Vec2::from_angle(rng.gen_range(0.0..TAU));

        commands.queue(SpawnAsteroid {
            asteroid: Asteroid {
                size: AsteroidSize::Large,
            },
            linear_velocity: LinearVelocity(
                velocity * angle,
            ),
            transform: Transform::from_xyz(
                bottom_right
                    .x
                    .lerp(top_left.x, rng.r#gen()),
                bottom_right
                    .y
                    .lerp(top_left.y, rng.r#gen()),
                0.,
            ),
        });
    }

    Ok(())
}

struct SpawnAsteroid {
    asteroid: Asteroid,
    linear_velocity: LinearVelocity,
    transform: Transform,
}

impl Command for SpawnAsteroid {
    fn apply(self, world: &mut World) -> () {
        let mut gizmo = GizmoAsset::default();

        gizmo.circle_2d(
            Isometry2d::default(),
            self.asteroid.size.radius(),
            SLATE_50,
        );

        let mut gizmo_assets = world
            .get_resource_mut::<Assets<GizmoAsset>>()
            .unwrap();
        let handle = gizmo_assets.add(gizmo);

        let collider =
            Collider::circle(self.asteroid.size.radius());
        world
            .commands()
            .spawn((
                self.asteroid.clone(),
                Gizmo {
                    handle: handle.clone(),
                    ..default()
                },
                collider,
                self.linear_velocity.clone(),
                self.transform.clone(),
            ))
            .observe(observe_asteroid_collision);
    }
}

fn observe_asteroid_collision(
    trigger: Trigger<OnCollisionStart>,
    asteroids: Query<(&Asteroid, &Transform)>,
    mut commands: Commands,
) {
    let Ok((asteroid, transform)) =
        asteroids.get(trigger.target())
    else {
        return;
    };
    let num_asteroids_to_spawn = asteroid.to_fragments();

    commands.entity(trigger.target()).despawn();

    for _ in 0..num_asteroids_to_spawn {
        use rand::{Rng, thread_rng};

        let mut rng = thread_rng();
        let velocity = rng.r#gen::<f32>() * 10.;
        let angle =
            Vec2::from_angle(rng.gen_range(0.0..TAU));

        // push the new asteroid in the direction it will travel
        let mut new_transform = transform.clone();
        let offset_distance = 20. * angle;
        new_transform.translation.x += offset_distance.x;
        new_transform.translation.y += offset_distance.y;

        commands.queue(SpawnAsteroid {
            asteroid: Asteroid {
                size: match asteroid.size {
                    AsteroidSize::Small => unreachable!(
                        "small asteroids don't spawn more"
                    ),
                    AsteroidSize::Medium => {
                        AsteroidSize::Small
                    }
                    AsteroidSize::Large => {
                        AsteroidSize::Medium
                    }
                },
            },
            linear_velocity: LinearVelocity(
                velocity * angle,
            ),
            transform: transform.clone(),
        });
    }
}

struct ShotTimer(Timer);

impl Default for ShotTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            0.5,
            TimerMode::Once,
        ))
    }
}

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Ship,     // Layer 1
    Asteroid, // Layer 2
}

#[derive(Component)]
struct Shot;

fn fire(
    input: Res<ButtonInput<KeyCode>>,
    mut timer: Local<ShotTimer>,
    time: Res<Time>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    mut commands: Commands,
    ship: Single<&Transform, With<Ship>>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if input.pressed(KeyCode::Space) {
        *timer = ShotTimer::default();
        let mut gizmo = GizmoAsset::default();
        gizmo.circle_2d(
            Isometry2d::default(),
            0.1,
            SLATE_50,
        );

        let mut transform = ship.into_inner().clone();

        transform.translation +=
            transform.rotation * Vec3::Y;

        commands
            .spawn((
                Shot,
                Gizmo {
                    handle: gizmo_assets.add(gizmo),
                    ..default()
                },
                transform,
                RigidBody::Dynamic,
                LinearVelocity(
                    (transform.rotation * Vec3::Y).xy()
                        * 10.,
                ),
                Collider::circle(0.1),
                CollisionLayers::new(
                    GameLayer::Ship,
                    [GameLayer::Asteroid],
                ),
                CollisionEventsEnabled,
            ))
            .observe(
                |trigger: Trigger<OnCollisionStart>,
                 mut commands: Commands| {
                    commands
                        .entity(trigger.target())
                        .despawn();
                },
            );
    }
}
