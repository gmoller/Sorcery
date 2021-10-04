use bevy::prelude::*;

use crate::enums::AppState;

pub fn check_for_end_turn_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        app_state.set(AppState::NPCsTurnState).unwrap();
        keyboard_input.reset(KeyCode::Return);
    }
}

pub fn player_turn_enter_system() {
    println!("player_turn_enter_system");
}

pub fn player_turn_exit_system() {
    println!("player_turn_exit_system");
}
