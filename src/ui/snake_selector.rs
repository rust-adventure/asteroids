use crate::{
    assets::{
        space::SpaceSheet, AudioAssets, FontAssets,
        ImageAssets,
    },
    colors,
    settings::{AudioSettings, GameSettings},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ShipIndex(usize);

pub fn snake_selector_interaction(
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    mut interaction_query: Query<
        (&Interaction, &ShipIndex),
        (Changed<Interaction>, With<Button>),
    >,
    sounds: Res<AudioAssets>,
) {
    for (interaction, snake_head) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if settings.audio == AudioSettings::ON {
                    // commands.spawn(AudioBundle {
                    //     source: sounds.apple.clone(),
                    //     ..default()
                    // });
                }
                settings.snake_index = snake_head.0;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
pub struct CurrentSnake;

pub fn update_current_snake(
    settings: ResMut<GameSettings>,
    mut image_query: Query<
        &mut TextureAtlas,
        With<CurrentSnake>,
    >,
) {
    for mut image in &mut image_query {
        image.index = settings.snake_index;
    }
}

pub fn spawn_snake_selector(
    parent: &mut ChildBuilder,
    images: Res<ImageAssets>,
    current_snake_index: usize,
    atlases: &Res<Assets<TextureAtlasLayout>>,
    fonts: &Res<FontAssets>,
    space_sheet_layout: SpaceSheet,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(25.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(25.0),
                        height: Val::Px(25.0),
                        margin: UiRect::right(Val::Px(
                            10.0,
                        )),
                        ..default()
                    },
                    image: images
                        .space_sheet
                        .clone()
                        .into(),
                    ..default()
                },
                TextureAtlas {
                    index: current_snake_index,
                    layout: space_sheet_layout.0.clone(),
                },
                CurrentSnake,
            ));
            parent.spawn(TextBundle::from_section(
                "Current Snake",
                TextStyle {
                    font: fonts.roboto.clone(),
                    font_size: 25.0,
                    color: colors::TEXT,
                },
            ));
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![
                    RepeatedGridTrack::flex(6, 1.),
                ],
                row_gap: Val::Px(3.),
                column_gap: Val::Px(3.),
                width: Val::Percent(100.),
                height: Val::Auto,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // let atlas = atlases
            //     .get(&space_sheet_layout.0)
            //     .expect(
            //     "snake texture_atlas_layout to be loaded",
            // );

            // for (i, _rect) in
            // atlas.textures.iter().enumerate().skip(108)
            for i in [200, 207, 214] {
                parent
                    .spawn((
                        ButtonBundle {
                            background_color: Color::NONE
                                .into(),
                            style: Style {
                                display: Display::Flex,
                                align_items:
                                    AlignItems::Center,
                                justify_self:
                                    JustifySelf::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ShipIndex(i),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: images
                                    .space_sheet
                                    .clone()
                                    .into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: i,
                                layout: space_sheet_layout
                                    .0
                                    .clone(),
                            },
                        ));
                    });
            }
        });
}
