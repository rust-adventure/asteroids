use bevy::prelude::*;

use crate::GameState;

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(3))
            .add_event::<RemoveLifeEvent>()
            .add_systems(
                Update,
                lives.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct Lives(usize);

#[derive(Event)]
pub struct RemoveLifeEvent;

fn lives(
    mut life_events: EventReader<RemoveLifeEvent>,
    mut lives: ResMut<Lives>,
) {
    for _event in life_events.read() {
        match lives.0.checked_sub(1) {
            Some(new_lives) => {
                lives.0 = new_lives;
            }
            None => {
                error!("GAME OVER");
            }
        }
    }
}
