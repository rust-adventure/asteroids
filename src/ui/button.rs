use super::MenuPage;
use crate::{
    assets::{
        space::SpaceSheet, AudioAssets, FontAssets,
        ImageAssets,
    },
    settings::{AudioSettings, GameSettings},
    GameState,
};
use bevy::{
    app::AppExit, ecs::system::Command, prelude::*,
};

const NORMAL_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 1.0,
};
const HOVERED_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.90,
    alpha: 1.0,
};
const PRESSED_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 80.0,
    alpha: 1.0,
};

#[derive(Component)]
pub struct TextButton;

pub fn text_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<TextButton>),
    >,
    text_query: Query<&Text>,
    mut exit: EventWriter<AppExit>,
    mut menu_page: ResMut<MenuPage>,
    settings: Res<GameSettings>,
    sounds: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, children) in
        &mut interaction_query
    {
        let text = text_query.get(children[0]).unwrap();
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
                *color = PRESSED_BUTTON.into();
                match text.sections[0].value.as_str() {
                    "New Game" => {
                        next_state
                            .set(GameState::ChooseShip);
                    }
                    "Settings" => {
                        *menu_page = MenuPage::Settings;
                        // Show Settings Page
                    }
                    "Exit" => {
                        exit.send(AppExit);
                    }
                    "Back" => {
                        *menu_page = MenuPage::Main;
                        // Show Main Menu Page
                    }
                    "Play" => {
                        next_state
                            .set(GameState::ChooseShip);
                    }
                    _ => {
                        unimplemented!(
                            "Button goes nowhere"
                        );
                    }
                }
            }
            Interaction::Hovered => {
                if settings.audio == AudioSettings::ON {
                    commands.spawn(AudioBundle {
                        source: sounds.menu_click.clone(),
                        ..default()
                    });
                }
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

pub struct SpawnButton<T: Into<String>> {
    pub text: T,
    pub parent: Entity,
}

impl<T: Into<String> + Send + 'static> Command
    for SpawnButton<T>
{
    fn apply(self, world: &mut World) {
        let font = world
            .get_resource::<FontAssets>()
            .unwrap()
            .alfa_slab_one_regular
            .clone();
        let space_sheet_layout = world
            .get_resource::<SpaceSheet>()
            .unwrap()
            .0
            .clone();
        let space_sheet = world
            .get_resource::<ImageAssets>()
            .unwrap()
            .space_sheet
            .clone();
        // images: Res<ImageAssets>,

        let panel_slicer = TextureSlicer {
            border: BorderRect::square(20.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };

        world
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(65.0),

                        // horizontally center child text
                        justify_content:
                            JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: NORMAL_BUTTON.into(),
                    image: space_sheet.into(),
                    ..default()
                },
                TextureAtlas {
                    index: 12,
                    layout: space_sheet_layout,
                },
                // ImageScaleMode::Sliced(
                //     panel_slicer.clone(),
                // ),
                TextButton,
            ))
            .set_parent(self.parent)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    self.text,
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::rgb(0.1, 0.1, 0.14),
                    },
                ));
            });
    }
}
