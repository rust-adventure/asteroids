use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use space_shooter::{
    assets::space::make_texture_atlas,
    laser_meteor_collision,
    meteors::MeteorPlugin,
    movement::MovementPlugin,
    start_game,
    ui::{
        choose_ship::ChooseShipPlugin,
        pause::{Pausable, PausePlugin},
    },
};
use space_shooter::{
    assets::AssetsPlugin, controls::ControlsPlugin,
    settings::SettingsPlugin, ui::UiPlugin, GameState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0., 0., 0.1,
        )))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroids!".into(),
                    ..default()
                }),
                ..default()
            }),
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
        ))
        .init_state::<GameState>()
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_systems(Startup, make_texture_atlas)
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(GameState::PlayingSandbox),
            start_game,
        )
        .add_systems(
            Update,
            laser_meteor_collision
                .run_if(in_state(GameState::PlayingSandbox))
                .run_if(resource_equals(
                    Pausable::NotPaused,
                )),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
