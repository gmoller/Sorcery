use std::collections::HashMap;

struct UnitType {
    id: u8,                // 0-255
    name: String,
    construction_cost: f32,
    number_of_figures: u8, // 0-255
    moves: f32,
    melee_attack: f32, // per figure
    defense: f32,      // per figure
    resistance: f32,   // per figure
    hit_points: f32    // per figure
}

pub(crate) struct UnitTypes {
    map: HashMap<u8, UnitType>
}

fn instantiate_unit_type(id: u8, name: String, construction_cost: f32, number_of_figures: u8, moves: f32, melee_attack: f32, defense: f32, resistance: f32, hit_points: f32) -> UnitType {
    let result = UnitType {
        id,
        name,
        construction_cost,
        number_of_figures,
        moves,
        melee_attack,
        defense,
        resistance,
        hit_points
    };

    return result;
}

pub(crate) fn load_unit_types() -> UnitTypes {
    let mut unit_types = HashMap::new();

    unit_types.insert( 1, instantiate_unit_type( 1, String::from("Settlers"), 60.0, 1, 1.0, 0.0, 1.0, 5.0, 10.0));
    unit_types.insert( 2, instantiate_unit_type( 2, String::from("Barbarian Spearmen"), 15.0, 8, 1.0, 1.0, 2.0, 5.0, 1.0));
    unit_types.insert( 3, instantiate_unit_type( 3, String::from("Barbarian Swordsmen"), 30.0, 6, 1.0, 3.0, 2.0, 5.0, 1.0));

    let result = UnitTypes { map: unit_types };

    return result;
}
