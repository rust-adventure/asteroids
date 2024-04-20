use bevy::prelude::*;

use crate::{
    assets::ImageAssets,
    kenney_assets::KenneySpriteSheetAsset,
    ship::PlayerShipType, GameState, Player,
};

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(3))
            .add_event::<RemoveLifeEvent>()
            .add_systems(
                Update,
                (lives, render_lives)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_life_ui,
            )
            .add_systems(
                OnExit(GameState::Playing),
                remove_life_ui,
            );
    }
}

#[derive(Resource, PartialEq, Eq)]
pub struct Lives(pub usize);

#[derive(Event)]
pub struct RemoveLifeEvent;

fn lives(
    mut life_events: EventReader<RemoveLifeEvent>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _event in life_events.read() {
        match lives.0.checked_sub(1) {
            Some(new_lives) => {
                lives.0 = new_lives;
                if lives.0 == 0 {
                    next_state.set(GameState::Menu);
                }
            }
            None => {
                next_state.set(GameState::Menu);
            }
        }
    }
}

/// LifeIndex is the order of lives
#[derive(Component)]
struct LifeIndex(usize);

#[derive(Component)]
struct LifeContainer;

fn spawn_life_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.)),
                column_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        },
        LifeContainer,
    ));
}
fn remove_life_ui(
    mut commands: Commands,
    query: Query<Entity, With<LifeContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
fn render_lives(
    mut commands: Commands,
    images: Res<ImageAssets>,
    lives: Res<Lives>,
    life_container: Query<Entity, With<LifeContainer>>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    player_query: Query<&PlayerShipType, With<Player>>,
    life_sprite_query: Query<(Entity, &LifeIndex)>,
) {
    let Ok(ship_type) = player_query.get_single() else {
        error_once!(
            "Only expected one PlayerShipType component. got {}",
            player_query.iter().count()
        );
        return;
    };

    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

    let container_id = life_container.single();

    for index in 0..lives.0 {
        // IF the live is currently shown
        if life_sprite_query.iter().any(
            |(_entity, life_index)| index == life_index.0,
        ) {
            // life already exists on screen, and should, continue;
            continue;
        } else {
            let next_life = commands
                .spawn((
                    ImageBundle {
                        image: space_sheet
                            .sheet
                            .clone()
                            .into(),
                        ..default()
                    },
                    TextureAtlas {
                        index: ship_type.life_atlas_index(),
                        layout: space_sheet
                            .texture_atlas_layout
                            .clone(),
                    },
                    LifeIndex(index),
                ))
                .id();
            commands
                .entity(container_id)
                .add_child(next_life);
        }
    }

    // remove unused lives
    for (entity, index) in &life_sprite_query {
        if index.0 >= lives.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
