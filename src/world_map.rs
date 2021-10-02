use std::collections::HashMap;
use bevy::prelude::*;

//use crate::constants::{HEX_OFFSET_Y, HEX_SIZE, LAYOUT_SIZE, SCALE};
use crate::constants::{HALF, HEX_EXTRA_Y, HEX_SIZE, LAYOUT_SIZE, SCALE};
use crate::hexagons::*;
use crate::resources::WorldMap;

pub(crate) fn spawn_map(commands: &mut Commands, world_map: &WorldMap, images: &HashMap<i32, Vec<Handle<ColorMaterial>>>) {
    // spawns the map into the ECS

    let scale = Vec3::new(SCALE.0, SCALE.1, 1.0);
    let size = Vec2::new(HEX_SIZE.0, HEX_SIZE.1 + HEX_EXTRA_Y);

    for row in 0..world_map.height {
        for column in 0..world_map.width {
            handle_tile(commands, column, row, &world_map, &images, size, scale);
        }
    }

}

fn handle_tile(commands: &mut Commands, column: u16, row: u16, world_map: &WorldMap, images: &HashMap<i32, Vec<Handle<ColorMaterial>>>, size: Vec2, scale: Vec3) {
    //

    let index = ((row * world_map.width) + column) as i32;
    let cell = &world_map.map[index as usize];
    let terrain_image = &cell.image;
    let value = images.get(&(terrain_image.texture_atlas_index as i32));
    if let Some(texture_atlas) = value {
        create_tile(commands, index, world_map, &terrain_image, texture_atlas, size, scale);
    }

}

fn create_tile(commands: &mut Commands, index: i32, world_map: &WorldMap, terrain_image: &crate::resources::Image, texture_atlas: &Vec<Handle<ColorMaterial>>, size: Vec2, scale: Vec3) {
    // 

    let axial = convert_index_to_axial(index, world_map.width); // convert index to axial
    let world_position = axial.hex_to_pixel(LAYOUT_SIZE, SCALE); // calculate world position from axial
    let position = Vec3::new(world_position.x as f32, world_position.y as f32 + HEX_EXTRA_Y * HALF * SCALE.1, (index as f32) / 1000.0);
    let image_index: usize = terrain_image.image_index.into();
    let image = texture_atlas[image_index].clone();
    let bundle = crate::create_bundles::create_sprite_bundle(size, position, scale, image);
    commands.spawn_bundle(bundle);

}

pub(crate) fn convert_index_to_axial(index: i32, map_width: u16) -> Hex { 
    // converts array index to an axial hex

    let row = index / (map_width as i32);
    let column = (index % (map_width as i32)) - (row / 2);

    let axial = Hex::new_axial(column, row);

    return axial;
}
