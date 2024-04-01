use bevy::{asset::LoadedAsset, utils::thiserror};
use bevy::{
    asset::{
        io::Reader, AssetLoader, AsyncReadExt, LoadContext,
    },
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use thiserror::Error;

/// Kenney makes [amazing assets](https://kenney.nl/).
///
/// Often these assets come with a spritesheet and an xml file describing said spritesheet.
pub struct KenneyAssetPlugin;

impl Plugin for KenneyAssetPlugin {
    fn build(&self, app: &mut App) {
        app
          .init_asset::<KenneySpriteSheetAsset>()
          .init_asset_loader::<KenneySpriteSheetAssetLoader>();
    }
}

#[derive(Debug)]
pub struct SubTexture {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Asset, TypePath, Debug)]
pub struct KenneySpriteSheetAsset {
    pub textures: Vec<SubTexture>,
    pub sheet: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Default)]
pub struct KenneySpriteSheetAssetLoader;

/// Possible errors that can be produced by [`KenneySpriteSheetAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum KenneySpriteSheetAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for KenneySpriteSheetAssetLoader {
    type Asset = KenneySpriteSheetAsset;
    type Settings = ();
    type Error = KenneySpriteSheetAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>>
    {
        Box::pin(async move {
            // original path must be the xml file
            let original_path =
                load_context.asset_path().path();
            let image_path =
                original_path.with_extension("png");
            let image = load_context
                .load_direct(image_path.clone())
                .await
                // todo: .unwrap
                .unwrap();
            let sheet_handle: Handle<Image> =
                load_context.load(image_path);

            let spritesheet_image: &Image =
                image.get().unwrap();
            let spritesheet_size =
                spritesheet_image.size_f32();

            let mut xml_string = String::new();
            reader
                .read_to_string(&mut xml_string)
                .await
                .unwrap();

            let doc =
                roxmltree::Document::parse(&xml_string)
                    .unwrap();

            let space_sheet_dimensions = Vec2::new(
                spritesheet_size.x,
                spritesheet_size.y,
            );

            let mut layout = TextureAtlasLayout::new_empty(
                space_sheet_dimensions,
            );
            let sub_textures: Vec<SubTexture> = doc
                .descendants()
                .filter(|element| {
                    element.tag_name()
                        == "SubTexture".into()
                })
                .map(|tex| {
                    let x: u32 = tex
                        .attribute("x")
                        .unwrap()
                        .parse()
                        .unwrap();
                    let y: u32 = tex
                        .attribute("y")
                        .unwrap()
                        .parse()
                        .unwrap();
                    let width: u32 = tex
                        .attribute("width")
                        .unwrap()
                        .parse()
                        .unwrap();
                    let height: u32 = tex
                        .attribute("height")
                        .unwrap()
                        .parse()
                        .unwrap();

                    layout.add_texture(Rect::from_corners(
                        Vec2::new(x as f32, y as f32),
                        Vec2::new(
                            (x + width) as f32,
                            (y + height) as f32,
                        ),
                    ));
                    SubTexture {
                        name: tex
                            .attribute("name")
                            .unwrap()
                            .to_string(),
                        x,
                        y,
                        width,
                        height,
                    }
                })
                .collect();
            let texture_atlas_layout =
                LoadedAsset::from(layout);
            let layout_handle = load_context
                .add_loaded_labeled_asset(
                    "texture_atlas_layout",
                    texture_atlas_layout,
                );
            Ok(KenneySpriteSheetAsset {
                textures: sub_textures,
                sheet: sheet_handle,
                texture_atlas_layout: layout_handle,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}
