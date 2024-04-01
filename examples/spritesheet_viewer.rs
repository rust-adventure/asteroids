// TODO: bevy_asset_loader currently doesn't support custom asset types.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use space_shooter::kenney_assets::{
    KenneyAssetPlugin, KenneySpriteSheetAsset,
};

fn main() {
    App::new()
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<ImageAssets>(),
        )
        .add_plugins((DefaultPlugins, KenneyAssetPlugin))
        .add_systems(OnEnter(MyStates::Next), setup)
        .add_systems(
            Update,
            input.run_if(in_state(MyStates::Next)),
        )
        .run()
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "spaceShooter2_spritesheet_2X.xml")]
    pub space_sheet: Handle<KenneySpriteSheetAsset>,
}

fn setup(
    mut commands: Commands,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
    images: Res<ImageAssets>,
) {
    let kenney_sheet =
        spritesheets.get(&images.space_sheet).unwrap();
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: kenney_sheet.sheet.clone(),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: kenney_sheet
                .texture_atlas_layout
                .clone(),
        },
    ));
}
fn input(
    input: Res<ButtonInput<KeyCode>>,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
    images: Res<ImageAssets>,
    mut atlas: Query<&mut TextureAtlas>,
) {
    let kenney_sheet =
        spritesheets.get(&images.space_sheet).unwrap();
    let mut atlas = atlas.single_mut();

    if input.just_pressed(KeyCode::Space) {
        atlas.index += 1;
    }
}
