use bevy::prelude::Entity;

use crate::hexagons::Hex;

pub struct IsMoving {
    pub destination_hex: Hex // axial hex destination of moving unit
}
impl IsMoving {
    pub fn new(destination_hex: Hex) -> IsMoving {
        return IsMoving { destination_hex };
    }
}

pub struct MainCameraTag;

pub struct SelectedTag;

pub struct ToBeSelectedTag;

pub struct Unit {
    pub unit_type_id: u16, // 0-65535
    pub location_hex: Hex, // axial hex location of unit
    pub movement_points: f32
}
impl Unit {
    pub fn new(unit_type_id: u16, location_hex: Hex, movement_points: f32) -> Unit {
        let unit = Unit {
            unit_type_id,
            location_hex,
            movement_points
        };

        return unit;
    }
}

pub struct UnitBadge {
    pub backdrop: Entity,
    pub backlight: Entity,
    pub unit_type: Entity,
    pub hp_fill: Entity,
    pub frame: Entity
}

pub struct OwnedByRace {
    pub race_type_id: u8
}
impl OwnedByRace {
    pub fn new(race_type_id: u8) -> OwnedByRace {
        return OwnedByRace { race_type_id };
    }
}
