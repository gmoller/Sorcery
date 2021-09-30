use bevy::prelude::Entity;

use crate::hexagons::Hex;

pub(crate) struct IsMoving {
    pub(crate) destination_hex: Hex // axial hex destination of moving unit
}
impl IsMoving {
    pub(crate) fn new(destination_hex: Hex) -> IsMoving {
        return IsMoving { destination_hex };
    }
}

pub(crate) struct MainCameraTag;

pub(crate) struct SelectedTag;

pub(crate) struct ToBeSelectedTag;

pub(crate) struct Unit {
    pub(crate) unit_type_id: u16, // 0-65535
    pub(crate) location_hex: Hex, // axial hex location of unit
    pub(crate) movement_points: f32
}
impl Unit {
    pub(crate) fn new(unit_type_id: u16, location_hex: Hex) -> Unit {
        return Unit { unit_type_id, location_hex, movement_points: 2.0 };
    }
}
impl Default for Unit {
    fn default() -> Self {
        Self::new(0, Hex::new_axial(-1, -1))
    }
}

pub(crate) struct UnitBadge {
    pub(crate) backdrop: Entity,
    pub(crate) backlight: Entity,
    pub(crate) unit_type: Entity,
    pub(crate) hp_fill: Entity,
    pub(crate) frame: Entity
}