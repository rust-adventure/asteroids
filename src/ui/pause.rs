use bevy::prelude::*;

use crate::{assets::ImageAssets, GameState};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_pause_toggle
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                show_pause_menu
                    .run_if(resource_changed::<Pausable>)
                    .run_if(resource_equals(
                        Pausable::Paused,
                    )),
                hide_pause_menu
                    .run_if(resource_changed::<Pausable>)
                    .run_if(resource_equals(
                        Pausable::NotPaused,
                    ))
                    .run_if(resource_exists::<PauseMenu>),
            ),
        )
        .insert_resource(Pausable::NotPaused);
    }
}

#[derive(Resource, PartialEq, Eq)]
pub enum Pausable {
    Paused,
    NotPaused,
}

#[derive(Resource)]
struct PauseMenu(Entity);

fn handle_pause_toggle(
    input: Res<ButtonInput<KeyCode>>,
    mut pausable: ResMut<Pausable>,
) {
    if input.just_pressed(KeyCode::Enter) {
        *pausable = match *pausable {
            Pausable::Paused => Pausable::NotPaused,
            Pausable::NotPaused => Pausable::Paused,
        }
    }
}

/// A [Condition](http://localhost:8000/bevy/ecs/prelude/trait.Condition.html) that enables systems running when the app is paused.
/// Likely used with [not](http://localhost:8000/bevy/ecs/schedule/common_conditions/fn.not.html)
///
/// ```rust
/// app.add_systems(my_system.run_if(not(paused)));
/// ```
pub fn paused() -> impl Condition<()> {
    IntoSystem::into_system(|paused: Res<Pausable>| {
        *paused == Pausable::Paused
    })
}

/// A one-shot system to pause the game
pub fn pause(mut pausable: ResMut<Pausable>) {
    *pausable = Pausable::Paused;
}

/// A one-shot system to unpause the game
pub fn unpause(mut pausable: ResMut<Pausable>) {
    *pausable = Pausable::NotPaused;
}

fn hide_pause_menu(
    mut commands: Commands,
    menu: Res<PauseMenu>,
) {
    commands.entity(menu.0).despawn_recursive();
}
fn show_pause_menu(
    mut commands: Commands,
    images: Res<ImageAssets>,
) {
    let panel_slicer = TextureSlicer {
        border: BorderRect::square(20.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let pause_text = commands
        .spawn((
            ImageBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                image: images.panel_glass.clone().into(),
                ..default()
            },
            ImageScaleMode::Sliced(panel_slicer.clone()),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Game Paused",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        })
        .id();

    let pause_menu_id = commands
        .spawn(NodeBundle {
            background_color: Color::rgba(
                0.95, 0.95, 1., 0.1,
            )
            .into(),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                width: Val::Percent(100.),
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        })
        .add_child(pause_text)
        .id();

    commands.insert_resource(PauseMenu(pause_menu_id));
}
