use bevy::prelude::*;

use space_shooter::kenney_assets::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, KenneyAssetPlugin))
        .init_resource::<State>()
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .add_systems(FixedUpdate, update)
        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<KenneySpriteSheetAsset>,
}

fn setup(
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>,
) {
    // Recommended way to load an asset
    state.handle = asset_server
        .load("spaceShooter2_spritesheet_2X.xml");

    // File extensions are optional, but are
    // recommended for project management and
    // last-resort inference
    // state.other_handle =
    //     asset_server.load("data/
    // asset_no_extension");
}

fn print_on_load(
    mut commands: Commands,
    state: ResMut<State>,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut printed: Local<bool>,
) {
    let custom_asset = spritesheets.get(&state.handle);

    if *printed || custom_asset.is_none() {
        return;
    }

    let kenney_sheet = custom_asset.unwrap();
    info!("image {:?}", kenney_sheet.sheet);
    info!(
        "first texture: {:?}",
        kenney_sheet.textures.first()
    );

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: kenney_sheet.sheet.clone(),
            ..default()
        },
        TextureAtlas {
            index: 60,
            layout: kenney_sheet
                .texture_atlas_layout
                .clone(),
        },
    ));
    // Once printed, we won't print again
    *printed = true;
}

fn update(
    mut atlas: Query<&mut TextureAtlas>,
    state: ResMut<State>,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
) {
    let custom_asset = spritesheets.get(&state.handle);

    for mut atlas in &mut atlas {
        let kenney_sheet = custom_asset.unwrap();

        if atlas.index + 1 == kenney_sheet.textures.len() {
            atlas.index = 0;
        } else {
            atlas.index += 1;
        }
    }
}
