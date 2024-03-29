use crate::{
    assets::{
        space::SpaceSheet, AudioAssets, FontAssets,
        ImageAssets,
    },
    colors,
    settings::{AudioSettings, GameSettings},
    GameState,
};
use bevy::prelude::*;

mod button;
pub mod choose_ship;
pub mod pause;
// mod snake_selector;
use self::{
    button::SpawnButton,
    // snake_selector::{
    //     snake_selector_interaction,
    // update_current_snake, },
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MenuPage::Main)
            .add_systems(PostStartup, pause_ui)
            .add_systems(
                OnEnter(GameState::Menu),
                show_menu,
            )
            .add_systems(OnExit(GameState::Menu), hide_menu)
            .add_systems(Update, button::text_button_system)
            .add_systems(
                Update,
                (
                    change_menu,
                    audio_state,
                    // snake_selector_interaction,
                    // update_current_snake,
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}

#[derive(Resource, Component, Debug, PartialEq)]
pub enum MenuPage {
    Main,
    Settings,
}

#[derive(Component)]
struct MainMenu;

fn show_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}
fn hide_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}
fn change_menu(
    menu: Res<MenuPage>,
    mut menu_pages: Query<(&MenuPage, &mut Visibility)>,
) {
    if menu.is_changed() {
        for (page, mut visibility) in menu_pages.iter_mut()
        {
            if page == &*menu {
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn audio_state(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (
            Changed<Interaction>,
            With<Button>,
            With<AudioSettingsCheckbox>,
        ),
    >,
    images: Res<ImageAssets>,
    mut settings: ResMut<GameSettings>,
    sounds: Res<AudioAssets>,
) {
    for (interaction, mut image) in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            if settings.audio == AudioSettings::ON {
                // commands.spawn(AudioBundle {
                //     source:
                // sounds.apple.clone(),
                //     ..default()
                // });
            }
            settings.audio = match settings.audio {
                AudioSettings::ON => AudioSettings::OFF,
                AudioSettings::OFF => AudioSettings::ON,
            };
            *image = UiImage::new(match settings.audio {
                AudioSettings::ON => {
                    images.box_checked.clone()
                }
                AudioSettings::OFF => {
                    images.box_unchecked.clone()
                }
            });
        }
    }
}

#[derive(Component)]
struct AudioSettingsCheckbox;

pub fn pause_ui(
    mut commands: Commands,
    images: Res<ImageAssets>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    fonts: Res<FontAssets>,
    space_sheet_layout: Res<SpaceSheet>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            let panel_slicer = TextureSlicer {
                border: BorderRect::square(20.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            };
     
            parent
                .spawn((
                    ImageBundle {
                        image: images.panel_glass.clone().into(),
                        style: Style {
                            width: Val::Px(360.0),
                            height: Val::Px(500.0),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::SpaceEvenly,
                            position_type:
                                PositionType::Absolute,
                            align_self: AlignSelf::Center,
                            border: UiRect::all(Val::Px(
                                10.0,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                    ImageScaleMode::Sliced(panel_slicer.clone()),
                    MenuPage::Main,
                ))
                .with_children(|parent| {
                    let entity = parent.parent_entity();
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "New Game"
                    });
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "Settings"
                    });
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "Exit"
                    });
                });
            parent
                .spawn((
                    ImageBundle {
                        image: images.panel_glass.clone().into(),
                        visibility: Visibility::Hidden,
                        style: Style {
                            width: Val::Px(360.0),
                            height: Val::Px(500.0),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::SpaceBetween,
                            border: UiRect::all(Val::Px(
                                10.0,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                    ImageScaleMode::Sliced(panel_slicer.clone()),
                    MenuPage::Settings,
                ))
                .with_children(|parent| {
                    let entity = parent.parent_entity();
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "Back"
                    });
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
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(25.0),
                                        height: Val::Px(25.0),
                                        margin:
                                            UiRect::right(
                                                Val::Px(
                                                    10.0,
                                                ),
                                            ),
                                        ..default()
                                    },
                                    image: UiImage::new(
                                        images
                                            .box_checked
                                            .clone(),
                                    ),
                                    ..default()
                                },
                                AudioSettingsCheckbox,
                            ));
                            parent.spawn(
                                TextBundle::from_section(
                                    "Play Audio",
                                    TextStyle {
                                        font:fonts.roboto.clone(),
                                        font_size: 25.0,
                                        color: colors::TEXT,
                                    },
                                ),
                            );
                        });

                    // snake_selector::spawn_snake_selector(
                    //     parent,
                    //     images,
                    //     0,
                    //     &atlases,
                    //     &fonts,
                    //     space_sheet_layout.clone()

                    // );
                });
        });
}
