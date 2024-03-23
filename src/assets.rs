use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub mod space;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .init_collection::<AudioAssets>()
            .init_collection::<FontAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "AlfaSlabOne-Regular.ttf")]
    pub alfa_slab_one_regular: Handle<Font>,
    #[asset(path = "roboto.ttf")]
    pub roboto: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "menu_click.ogg")]
    pub menu_click: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "grey_box.png")]
    pub box_unchecked: Handle<Image>,
    #[asset(path = "green_boxCheckmark.png")]
    pub box_checked: Handle<Image>,
    #[asset(path = "pattern_blueprint.png")]
    pub pattern_blueprint: Handle<Image>,
    #[asset(path = "space_sheet.png")]
    pub space_sheet: Handle<Image>,
}
