use std::collections::HashMap;

use crate::constants::{UNIT_ICON_SETTLERS_TRANSPARENT, UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT, UNIT_ICON_BARBARIAN_SWORDSMEN_TRANSPARENT};

pub struct UnitType {
    id: u16,            // 0-65535
    name: String,
    construction_cost: f32,
    number_of_figures: u8, // 0-255
    pub moves: f32,
    melee_attack: f32, // per figure
    defense: f32,      // per figure
    resistance: f32,   // per figure
    hit_points: f32,    // per figure
    pub image_id: u8
}

pub struct UnitTypes {
    map: HashMap<u16, UnitType>
}
impl UnitTypes {
    pub fn get_by_id(&self, unit_type_id: u16) -> &UnitType {
        let ut = self.map.get(&unit_type_id);
        let result = match ut {
            None => panic!("Unit type with id [{}] not found", unit_type_id),
            Some(unit_type) => unit_type
        };

        return result;
    }
    pub fn clone(&self) -> UnitTypes {
        let mut map = HashMap::new();
        for item in &self.map {
            let key = item.0.clone();
            let value = UnitType { id: item.1.id, name: item.1.name.clone(), construction_cost: item.1.construction_cost, number_of_figures: item.1.number_of_figures, moves: item.1.moves, melee_attack: item.1.melee_attack, defense: item.1.defense, resistance: item.1.resistance, hit_points: item.1.hit_points, image_id: item.1.image_id };
            map.insert(key, value);
        }

        let result = UnitTypes { map };

        return result;
    }
}

fn instantiate_unit_type(id: u16, name: String, construction_cost: f32, number_of_figures: u8, moves: f32, melee_attack: f32, defense: f32, resistance: f32, hit_points: f32, image_id: u8) -> UnitType {
    let result = UnitType {
        id,
        name,
        construction_cost,
        number_of_figures,
        moves,
        melee_attack,
        defense,
        resistance,
        hit_points,
        image_id
    };

    return result;
}

pub fn load_unit_types() -> UnitTypes {
    let mut unit_types = HashMap::new();

    unit_types.insert( 1, instantiate_unit_type( 1, String::from("Settlers"), 60.0, 1, 1.0, 0.0, 1.0, 5.0, 10.0, UNIT_ICON_SETTLERS_TRANSPARENT));
    unit_types.insert( 2, instantiate_unit_type( 2, String::from("Barbarian Spearmen"), 15.0, 8, 1.0, 1.0, 2.0, 5.0, 1.0, UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT));
    unit_types.insert( 3, instantiate_unit_type( 3, String::from("Barbarian Swordsmen"), 30.0, 6, 1.0, 3.0, 2.0, 5.0, 1.0, UNIT_ICON_BARBARIAN_SWORDSMEN_TRANSPARENT));

    let result = UnitTypes { map: unit_types };

    return result;
}
