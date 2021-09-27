use std::collections;
use bevy::prelude::*;

use crate::{HEX_OFFSET_Y, HEX_SIZE, LAYOUT_SIZE, SCALE, THREE_QUARTERS};

pub(crate) fn spawn_unit(commands: &mut Commands, images: &collections::HashMap<i32, Vec<Handle<ColorMaterial>>>, location_hex_axial: crate::hexagons::Hex, image_id: u8, as_to_be_selected: bool, as_selected: bool) {    
    // spawns a unit entity into the ECS

    let world_position = location_hex_axial.hex_to_pixel(LAYOUT_SIZE, SCALE); // calculate world position from axial

    let value = images.get(&(6));
    if let Some(texture_atlas) = value {
        let color_material_handle = texture_atlas[image_id as usize].clone();

        let sprite_dimensions = Vec2::new(HEX_SIZE.0 * THREE_QUARTERS, (HEX_SIZE.1 - HEX_OFFSET_Y * 2.0) * THREE_QUARTERS);
        let sprite_scale = Vec3::new(SCALE.0 as f32, SCALE.1 as f32, 1.0);

        let position = Vec3::new(world_position.x as f32, world_position.y as f32, 10.0);

        let bundle = crate::create_bundles::create_sprite_bundle(sprite_dimensions, position, sprite_scale, color_material_handle);

        let mut entity = commands.spawn_bundle(bundle);
        let entity = entity.insert(crate::components::Unit::new(1, location_hex_axial));

        if as_to_be_selected {
            entity.insert(crate::components::ToBeSelectedTag);
        }

        if as_selected {
            entity.insert(crate::components::SelectedTag);
        }
    }

}
