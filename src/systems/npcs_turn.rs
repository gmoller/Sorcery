use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::components::{IsMoving, NeedsOrders, OwnedByFaction, Unit};
use crate::enums::AppState;
use crate::resources::WorldMap;

pub fn give_orders_system(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut unit_query: Query<(Entity, &mut Unit, &OwnedByFaction), With<NeedsOrders>>
) {
    for (entity, mut unit, owned_by_faction) in unit_query.iter_mut() {
        
        if owned_by_faction.faction_id == 1 { continue; }

        if unit.movement_points >= 1.0 {
            let neighbors = unit.location_hex.all_hex_neighbors();
            let mut rng = thread_rng();
            let index = rng.gen_range(0..6);
            let hex_to_move_to = neighbors[index];
    
            let off_map = crate::systems::unit_systems::check_if_off_map(hex_to_move_to, world_map.width, world_map.height);
            if !off_map {
                let is_moving = IsMoving::new(hex_to_move_to);
                commands.entity(entity).insert(is_moving);

                unit.movement_points -= 1.0;
            }
        }

        commands.entity(entity).remove::<NeedsOrders>();
    }
}

pub fn check_for_end_turn_system(
    mut app_state: ResMut<State<AppState>>,
    unit_query: Query<&OwnedByFaction, With<NeedsOrders>>
) {

    let mut any_without_orders = false;
    for owned_by_faction in unit_query.iter() {

        if owned_by_faction.faction_id == 1 { continue; }

        any_without_orders = true;
        break;
    }

    let all_have_been_given_orders = !any_without_orders;
    if all_have_been_given_orders {
        app_state.set(AppState::StartNewTurnState).unwrap();
    }
}

pub fn npcs_turn_enter_system() {
    println!("npcs_turn_enter_system");
}

pub fn npcs_turn_exit_system() {
    println!("npcs_turn_exit_system");
}
