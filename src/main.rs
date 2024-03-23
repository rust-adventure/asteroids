use bevy::prelude::*;
use space_shooter::{
    assets::space::make_texture_atlas, start_game,
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space!".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_systems(Startup, make_texture_atlas)
        .add_plugins((
            SettingsPlugin,
            ControlsPlugin,
            AssetsPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(GameState::Playing),
            start_game,
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
