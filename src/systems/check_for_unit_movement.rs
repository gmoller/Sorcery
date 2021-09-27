use bevy::{prelude::*, render::camera::Camera};

use crate::{HALF, LAYOUT_SIZE, SCALE, hexagons::{Hex, Point}};

pub(crate) fn check_for_unit_movement(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
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
    let fractional_hex = crate::hexagons::pixel_to_hex_pointy_layout(world_position, LAYOUT_SIZE, SCALE);
    let world_hex = crate::hexagons::hex_round(fractional_hex);

    return world_hex;
}
