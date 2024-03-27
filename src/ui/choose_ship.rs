use bevy::prelude::*;

use crate::{
    assets::{
        space::SpaceSheet, AudioAssets, FontAssets,
        ImageAssets,
    },
    settings::{AudioSettings, GameSettings},
    ship::{PlayerShipType, SpawnFrom},
    GameState,
};

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
    space_sheet_layout: Res<SpaceSheet>,
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
    let ships: Vec<_> = PlayerShipType::all_ships()
        .into_iter()
        .map(|ship_type| {
            let ship = commands
                .spawn((
                    ImageBundle {
                        image: images
                            .space_sheet
                            .clone()
                            .into(),
                        ..default()
                    },
                    TextureAtlas {
                        index: ship_type.base_atlas_index(),
                        layout: space_sheet_layout
                            .0
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
                commands.insert_resource(ship_type.clone());
                commands.insert_resource(SpawnFrom(
                    transform.clone(),
                ));
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
