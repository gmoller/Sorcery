use std::collections::HashMap;
use bevy::{prelude::*, render::camera::Camera};

use crate::constants::{HALF, LAYOUT_SIZE, SCALE, UNIT_FRAME_ACTIVE, UNIT_FRAME_HOVERED, UNIT_FRAME_INACTIVE, UNIT_MOVEMENT_SPEED};
use crate::components::{IsMoving, MainCameraTag, SelectedTag, ToBeSelectedTag, Unit, UnitBadge};
use crate::hexagons::{Hex, Point};
use crate::resources::{ScreenSize, WorldMap};

pub(crate) fn select_unit_system(
    mut commands: Commands,
    images: Res<HashMap<i32, Vec<Handle<ColorMaterial>>>>,
    mut unit_query: Query<(Entity, &Unit, &UnitBadge), With<ToBeSelectedTag>>,
    mut color_material_query: Query<&mut Handle<ColorMaterial>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<MainCameraTag>)>
) {
    // focuses camera on unit that is selected and changes unit to active

    let mut camera_transform = camera_query.single_mut().unwrap();

    for (entity, unit, unit_badge) in unit_query.iter_mut() {

        // focus camera on unit
        let world_position = unit.location_hex.hex_to_pixel(LAYOUT_SIZE, SCALE); // calculate world position from hex
        camera_transform.translation.x = world_position.x as f32;
        camera_transform.translation.y = world_position.y as f32;

        // change unit to active
        change_unit_frame(&mut color_material_query, unit_badge, &images, UNIT_FRAME_ACTIVE);

        commands.entity(entity).remove::<ToBeSelectedTag>();
        commands.entity(entity).insert(SelectedTag);
        
    }

}

pub(crate) fn check_for_unit_movement_system(
    mut commands: Commands,
    screen_size: Res<ScreenSize>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    world_map: Res<WorldMap>,
    query: Query<(Entity, &Unit), (With<SelectedTag>, Without<IsMoving>)>,
    mut camera_query: Query<&Transform, (With<Camera>, With<MainCameraTag>)>
) {
    // if a unit is selected and mouse right clicked in a neighbouring hex
    // then set unit to IsMoving and set destination

    if mouse_input.just_pressed(MouseButton::Right) {

        let camera_transform = camera_query.single_mut().unwrap();

        for (entity, unit) in query.iter() {
            if unit.movement_points >= 1.0 {
                if let Some(hex) = get_hex_clicked_on(&windows, &screen_size, camera_transform) {
                    let neighbors = unit.location_hex.all_hex_neighbors();
    
                    for item in neighbors {
                        if item == hex {
                            let off_map = check_if_off_map(hex, world_map.width, world_map.height);
                            if off_map { break; }
        
                            let is_moving = IsMoving::new(hex);
                            commands.entity(entity).insert(is_moving);
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn check_for_unit_selection_system(
    mut commands: Commands,
    screen_size: Res<ScreenSize>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    unit_query: Query<(Entity, &Unit), Without<SelectedTag>>,
    mut camera_query: Query<&Transform, (With<Camera>, With<MainCameraTag>)>
) {
    // if a unit is unselected and mouse left clicked in the hex that the unit is on
    // then set unit to selected

    if mouse_input.just_pressed(MouseButton::Left) {

        let camera_transform = camera_query.single_mut().unwrap();

        for (entity, unit) in unit_query.iter() {
            if let Some(hex) = get_hex_clicked_on(&windows, &screen_size, camera_transform) {
                if unit.location_hex == hex {
                    commands.entity(entity).insert(ToBeSelectedTag);
                }
            }
        }
    }
}

pub(crate) fn check_for_unit_unselection_system(
    mut commands: Commands,
    screen_size: Res<ScreenSize>,
    images: Res<HashMap<i32, Vec<Handle<ColorMaterial>>>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    unit_query: Query<(Entity, &Unit, &UnitBadge), With<SelectedTag>>,
    mut color_material_query: Query<&mut Handle<ColorMaterial>>,
    mut camera_query: Query<&Transform, (With<Camera>, With<MainCameraTag>)>
) {
    // if a unit is selected and mouse left clicked in a hex other than the one the selected unit is on
    // then set unit to unselected

    if mouse_input.just_pressed(MouseButton::Left) {

        let camera_transform = camera_query.single_mut().unwrap();

        for (entity, unit, unit_badge) in unit_query.iter() {
            if let Some(hex) = get_hex_clicked_on(&windows, &screen_size, camera_transform) {
                if unit.location_hex != hex {
                    // change unit to inactive
                    change_unit_frame(&mut color_material_query, unit_badge, &images, UNIT_FRAME_INACTIVE);

                    commands.entity(entity).remove::<SelectedTag>();
                }
            }
        }
    }

}

pub(crate) fn check_for_unit_hover_system(
    images: Res<HashMap<i32, Vec<Handle<ColorMaterial>>>>,
    screen_size: Res<ScreenSize>,
    windows: Res<Windows>,
    unit_query: Query<(&Unit, &UnitBadge), (Without<SelectedTag>, Without<IsMoving>)>,
    mut color_material_query: Query<&mut Handle<ColorMaterial>>,
    mut camera_query: Query<&Transform, (With<Camera>, With<MainCameraTag>)>
) {
    // if a unit is unselected and mouse cursor is in the hex of the unit
    // then set unit's badge to hovering

    let camera_transform = camera_query.single_mut().unwrap();

    for (unit, unit_badge) in unit_query.iter() {
        if let Some(hex) = get_hex_clicked_on(&windows, &screen_size, camera_transform) {
            if unit.location_hex == hex {
                // change unit to hovered
                change_unit_frame(&mut color_material_query, unit_badge, &images, UNIT_FRAME_HOVERED);
            } else {
                // change unit to inactive
                change_unit_frame(&mut color_material_query, unit_badge, &images, UNIT_FRAME_INACTIVE);
            }
        }
    }
}

pub(crate) fn move_unit_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut crate::components::Unit, &crate::components::IsMoving)>
) {
    // if a unit is moving, adjust it's transform to move closer to it's destination
    // if it has arrived at it's destination, remove the IsMoving component and set it's location

    for (entity, mut unit_transform, mut unit, is_moving) in query.iter_mut() {

        let unit_translation = &mut unit_transform.translation;
        let current_point = Point::new(unit_translation.x.into(), unit_translation.y.into());
        let desintation_point = is_moving.destination_hex.hex_to_pixel(LAYOUT_SIZE, SCALE);
        //println!("current_point: {:?}", current_point);
        //println!("desintation_point: {:?}", desintation_point);
        let direction: Vec2 = calculate_direction(current_point, desintation_point);

        unit_translation.x += direction.x * time.delta_seconds();
        unit_translation.y += direction.y * time.delta_seconds();

        // TODO: if unit badge is off screen, move camera to keep unit on screen

        let current_point = Point::new(unit_translation.x.into(), unit_translation.y.into());
        let arrived_at_destination = arrived_at_destination(current_point, desintation_point, direction);
        if arrived_at_destination {
            unit.location_hex = is_moving.destination_hex;
            commands.entity(entity).remove::<IsMoving>();
            unit_translation.x = desintation_point.x as f32;
            unit_translation.y = desintation_point.y as f32;

            unit.movement_points -= 1.0;
        }

    }

}

fn change_unit_frame(color_material_query: &mut Query<&mut Handle<ColorMaterial>>, unit_badge: &UnitBadge, images: &Res<HashMap<i32, Vec<Handle<ColorMaterial>>>>, image_id: u8) {
    if let Ok(mut material) = color_material_query.get_mut(unit_badge.frame) {
        if let Some(texture_atlas) = images.get(&(6)) { // 6: Units
            let color_material_handle_unit_frame = texture_atlas[image_id as usize].clone();
            *material = color_material_handle_unit_frame;
        } else {
            panic!("Image not found!");
        }
    } else {
        panic!("Image not found!");
    }
}

fn get_hex_clicked_on(windows: &Res<Windows>, screen_size: &Res<ScreenSize>, camera_transform: &Transform) -> Option<Hex> {

    let window = windows.get_primary().expect("no primary window");

    if let Some(cursor_position) = window.cursor_position() {
        let hex = translate_mouse_position_to_world_hex(cursor_position, screen_size, camera_transform);

        return Some(hex);
    }

    return None;
}

fn translate_mouse_position_to_world_hex(mouse_cursor_position: Vec2, screen_size: &Res<ScreenSize>, camera_transform: &Transform) -> Hex {

    let screen_position = Vec2::new(mouse_cursor_position.x - screen_size.width * HALF, mouse_cursor_position.y - screen_size.height * HALF);
    let world_position = camera_transform.compute_matrix() * screen_position.extend(0.0).extend(1.0);
    //println!("World coords: {}/{}", world_position.x, world_position.y);

    let fractional_hex = Point::new(world_position.x as f64, world_position.y as f64).pixel_to_hex(LAYOUT_SIZE, SCALE);
    let world_hex = fractional_hex.round();

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

fn calculate_direction(current_point: crate::hexagons::Point, desintation_point: crate::hexagons::Point) -> Vec2 {
    // calculates the direction the unit needs to move in

    let x = (desintation_point.x - current_point.x) as f32;
    let y = (desintation_point.y - current_point.y) as f32;
    let vec = Vec2::new(x, y);
    let normalized = vec.normalize();
    let result = normalized * UNIT_MOVEMENT_SPEED;

    return result;
}

fn arrived_at_destination(current_point: crate::hexagons::Point, desintation_point: crate::hexagons::Point, direction: Vec2) -> bool {
    // return true if: current_point == desintation_point or greater

    let result: bool;
    if direction.x >= 0.0 && direction.y >= 0.0 {
        result = current_point.x >= desintation_point.x && current_point.y >= desintation_point.y;
    } else if direction.x >= 0.0 && direction.y < 0.0 {
        result = current_point.x >= desintation_point.x && current_point.y <= desintation_point.y;
    }
    else if direction.x < 0.0 && direction.y < 0.0 {
        result = current_point.x <= desintation_point.x && current_point.y <= desintation_point.y;
    } else if direction.x < 0.0 && direction.y >= 0.0 {
        result = current_point.x <= desintation_point.x && current_point.y >= desintation_point.y;
    } else {
        panic!("I don't know where I'm going!");
    }

    return result;
}
