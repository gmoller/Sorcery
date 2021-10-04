use bevy::prelude::*;

use crate::enums::AppState;

pub fn check_for_end_turn_system(
    mut app_state: ResMut<State<AppState>>
) {
    app_state.set(AppState::StartNewTurnState).unwrap();
}

pub fn npcs_turn_enter_system() {
    println!("npcs_turn_enter_system");
}

pub fn npcs_turn_exit_system() {
    println!("npcs_turn_exit_system");
}
