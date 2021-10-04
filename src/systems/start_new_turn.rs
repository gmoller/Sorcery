use bevy::prelude::*;

use crate::config::units::UnitTypes;
use crate::{enums::AppState};
use crate::components::{OwnedByWizard, Unit};

pub fn start_new_turn_update_system(
    mut app_state: ResMut<State<AppState>>,
    unit_types: Res<UnitTypes>,
    mut unit_query: Query<(&mut Unit, &OwnedByWizard)>,
) {
    //TODO: add new turn logic here
    // reset unit movement points
    for (mut unit, owned_by_wizard) in unit_query.iter_mut() {

        if owned_by_wizard.wizard_id != 1 { continue; }

        unit.reset_movement_points(&unit_types);
    }

    app_state.set(AppState::PlayerTurnState).unwrap();
}

pub fn start_new_turn_enter_system() {
    println!("start_new_turn_enter_system");
}

pub fn start_new_turn_exit_system() {
    println!("start_new_turn_exit_system");
}
