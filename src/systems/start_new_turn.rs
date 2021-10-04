use bevy::prelude::*;

use crate::config::units::UnitTypes;
use crate::components::{NeedsOrders, Unit};
use crate::enums::AppState;

pub fn start_new_turn_update_system(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    unit_types: Res<UnitTypes>,
    mut unit_query: Query<(Entity, &mut Unit)>,
) {
    // reset unit movement points and add NeedsOrders tag
    for (entity, mut unit) in unit_query.iter_mut() {

        //if owned_by_faction.faction_id != 1 { continue; }

        unit.reset_movement_points(&unit_types);
        commands.entity(entity).insert(NeedsOrders);
    }

    app_state.set(AppState::PlayerTurnState).unwrap();
}

pub fn start_new_turn_enter_system() {
    println!("start_new_turn_enter_system");
}

pub fn start_new_turn_exit_system() {
    println!("start_new_turn_exit_system");
}
