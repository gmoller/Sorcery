use bevy::{prelude::*, render::camera::Camera};

use crate::{HALF, LAYOUT_SIZE, SCALE};
use crate::hexagons::{Hex, Point};

pub(crate) fn check_for_unit_movement(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    world_map: Res<crate::resources::WorldMap>,
    query: Query<(Entity, &crate::components::Unit), (With<crate::components::SelectedTag>, Without<crate::components::IsMoving>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<crate::components::MainCameraTag>)>
) {
    // if a unit is selected and mouse right clicked in a neighbouring hex
    // then set unit to IsMoving and set destination

    if mouse_input.just_pressed(MouseButton::Right) {

        let camera_query = camera_query.single_mut().unwrap();
        let camera_x: f64 = camera_query.translation.x as f64;
        let camera_y: f64 = camera_query.translation.y as f64;
        for (entity, unit) in query.iter() {
            let win = windows.get_primary().expect("no primary window");
            let screen_width = win.width();
            let screen_height = win.height();
            let value = win.cursor_position();
            if let Some(cursor_position) = value {
                let hex = translate_mouse_position_to_world_hex(cursor_position, screen_width, screen_height, camera_x, camera_y);
                //println!("selected hex at {:?}", hex);
                //println!("unit at {:?}", unit.location_hex);

                let neighbors = crate::hexagons::all_hex_neighbors(unit.location_hex);

                for item in neighbors {
                    if item == hex {
                        let off_map = check_if_off_map(hex, world_map.width, world_map.height);
                        if off_map { break; }

                        let is_moving = crate::components::IsMoving::new(hex);
                        commands.entity(entity).insert(is_moving);
                        //println!("desination hex {:?}", hex);
                        break;
                    }
                }
            }
        }

    }

}

fn translate_mouse_position_to_world_hex(mouse_cursor_position: Vec2, screen_width: f32, screen_height: f32, camera_x: f64, camera_y: f64) -> Hex {

    let screen_position = Point::new((mouse_cursor_position.x - screen_width * HALF) as f64, (mouse_cursor_position.y - screen_height * HALF) as f64);
    let world_position = Point::new(screen_position.x + camera_x, screen_position.y + camera_y);
    let fractional_hex = world_position.pixel_to_hex(LAYOUT_SIZE, SCALE);
    let world_hex = crate::hexagons::hex_round(fractional_hex);

    return world_hex;
}

fn check_if_off_map(hex: Hex, world_map_width: u16, world_map_height: u16) -> bool {

    if hex.r < 0 { return true; }
    if hex.r > (world_map_height as i32 - 1) { return true; }

    let min_q_for_row = (hex.r / 2) * -1;
    let max_q_for_row = min_q_for_row + (world_map_width as i32 - 1);

    if hex.q < min_q_for_row { return true;}
    if hex.q > max_q_for_row { return true;}

    return false;
}