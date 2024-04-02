use bevy::prelude::*;

use crate::{
    assets::{AudioAssets, ImageAssets},
    kenney_assets::KenneySpriteSheetAsset,
    settings::{AudioSettings, GameSettings},
    ship::PlayerShipType,
    GameState,
};

pub struct ChooseShipPlugin;

impl Plugin for ChooseShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChooseShipEvent>()
            .add_systems(
                OnEnter(GameState::ChooseShip),
                choose_ship_menu,
            )
            .add_systems(
                Update,
                choose_ship_button_system.run_if(in_state(
                    GameState::ChooseShip,
                )),
            )
            .add_systems(
                OnExit(GameState::ChooseShip),
                hide_ship_menu,
            );
    }
}

#[derive(Event)]
pub struct ChooseShipEvent {
    pub ship_type: PlayerShipType,
    pub ship_menu_location: Transform,
}

#[derive(Component)]
pub struct ChooseShipMenu;

#[derive(Debug, Component)]
pub struct ShipIndex(pub usize);

pub fn hide_ship_menu(
    mut choose_ship_menu: Query<
        &mut Visibility,
        With<ChooseShipMenu>,
    >,
) {
    for mut visibility in &mut choose_ship_menu {
        *visibility = Visibility::Hidden;
    }
}
pub fn choose_ship_menu(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut choose_ship_menu: Query<
        &mut Visibility,
        With<ChooseShipMenu>,
    >,
) {
    if !choose_ship_menu.is_empty() {
        let mut visibility = choose_ship_menu.single_mut();
        *visibility = Visibility::Visible;
        return;
    }
    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();
    let ships: Vec<_> = PlayerShipType::all_ships()
        .into_iter()
        .map(|ship_type| {
            let ship = commands
                .spawn((
                    ImageBundle {
                        image: space_sheet
                            .sheet
                            .clone()
                            .into(),
                        ..default()
                    },
                    TextureAtlas {
                        index: ship_type.base_atlas_index(),
                        layout: space_sheet
                            .texture_atlas_layout
                            .clone(),
                    },
                ))
                .id();
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            justify_content:
                                JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        image: images
                            .pattern_blueprint
                            .clone()
                            .into(),
                        ..default()
                    },
                    ImageScaleMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 0.5,
                    },
                    ship_type,
                ))
                .add_child(ship)
                .id()
        })
        .collect();

    let mut wrapper = commands.spawn((
        NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        },
        ChooseShipMenu,
    ));

    for ship in ships {
        wrapper.add_child(ship);
    }
}

pub fn choose_ship_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &PlayerShipType,
            &Transform,
        ),
        Changed<Interaction>,
    >,
    settings: Res<GameSettings>,
    sounds: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut choose_ship_events: EventWriter<ChooseShipEvent>,
) {
    for (interaction, ship_type, transform) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if settings.audio == AudioSettings::ON {
                    // commands.spawn(AudioBundle
                    // {
                    //     source:
                    // sounds.apple.clone(),
                    //     ..default()
                    // });
                }
                // *color = PRESSED_BUTTON.into();

                choose_ship_events.send(ChooseShipEvent {
                    ship_type: ship_type.clone(),
                    ship_menu_location: *transform,
                });
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                if settings.audio == AudioSettings::ON {
                    commands.spawn(AudioBundle {
                        source: sounds.menu_click.clone(),
                        ..default()
                    });
                }
                // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // *color = NORMAL_BUTTON.into();
            }
        }
    }
}
