use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioSettings {
    ON,
    OFF,
}

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct GameSettings {
    pub audio: AudioSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            audio: AudioSettings::ON,
        }
    }
}
