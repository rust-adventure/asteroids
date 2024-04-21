use bevy::{
    prelude::*,
    render::{
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_hanabi::prelude::*;
use bevy_xpbd_2d::prelude::*;
use space_shooter::{
    assets::AssetsPlugin, controls::ControlsPlugin,
    levels::LevelsPlugin, lives::LifePlugin, reset_game,
    settings::SettingsPlugin, ship::ShipPlugin,
    ship_meteor_collision, ui::UiPlugin, GameState,
};
use space_shooter::{
    laser_meteor_collision,
    meteors::MeteorPlugin,
    movement::MovementPlugin,
    start_game,
    ui::{
        choose_ship::ChooseShipPlugin,
        pause::{Pausable, PausePlugin},
    },
};

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features.set(
        WgpuFeatures::VERTEX_WRITABLE_STORAGE,
        true,
    );

    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0., 0., 0.1,
        )))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Asteroids!".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                    synchronous_pipeline_compilation: false,
                })
                .set(ImagePlugin::default_nearest()),
            SettingsPlugin,
            ControlsPlugin,
            AssetsPlugin,
            UiPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            MeteorPlugin,
            MovementPlugin,
            ChooseShipPlugin,
            PausePlugin,
            HanabiPlugin,
            ShipPlugin,
            LifePlugin,
            LevelsPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(GameState::Playing),
            (reset_game, start_game).chain(),
        )
        .add_systems(
            Update,
            (
                laser_meteor_collision,
                ship_meteor_collision,
            )
                .run_if(in_state(GameState::Playing))
                .run_if(resource_equals(
                    Pausable::NotPaused,
                )),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
