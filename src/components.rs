use bevy::prelude::*;

use crate::{config::units::UnitTypes, hexagons::Hex};

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
    pub fn new(unit_type_id: u16, location_hex: Hex, unit_types: &UnitTypes) -> Unit {
        let mut unit = Unit {
            unit_type_id,
            location_hex,
            movement_points: 0.0
        };
        unit.reset_movement_points(&unit_types);

        return unit;
    }
    pub fn reset_movement_points(&mut self, unit_types: &UnitTypes) {

        let unit_type = unit_types.get_by_id(self.unit_type_id);
        self.movement_points = unit_type.moves;
    }
}

pub struct UnitBadge {
    pub backdrop: Entity,
    pub backlight: Entity,
    pub unit_type: Entity,
    pub hp_fill: Entity,
    pub frame: Entity
}

pub struct OwnedByFaction {
    pub faction_id: u8
}
impl OwnedByFaction {
    pub fn new(faction_id: u8) -> OwnedByFaction {
        return OwnedByFaction { faction_id };
    }
}

pub struct NeedsOrders;