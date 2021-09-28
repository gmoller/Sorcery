use bevy::prelude::*;

use crate::{LAYOUT_SIZE, SCALE};
use crate::components::IsMoving;
use crate::hexagons::Point;

const SPEED: f32 = 400.0 * SCALE.0;

pub(crate) fn move_unit(
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

        let current_point = Point::new(unit_translation.x.into(), unit_translation.y.into());
        let arrived_at_destination = arrived_at_destination(current_point, desintation_point, direction);
        if arrived_at_destination {
            unit.location_hex = is_moving.destination_hex;
            commands.entity(entity).remove::<IsMoving>();
            unit_translation.x = desintation_point.x as f32;
            unit_translation.y = desintation_point.y as f32;

            // TODO: if unit badge is off screen, focus camera on unit
            // TODO: subtract movement_points
        }

    }

}

fn calculate_direction(current_point: crate::hexagons::Point, desintation_point: crate::hexagons::Point) -> Vec2 {
    // calculates the direction the unit needs to move in

    let x = (desintation_point.x - current_point.x) as f32;
    let y = (desintation_point.y - current_point.y) as f32;
    let vec = Vec2::new(x, y);
    let normalized = vec.normalize();
    let result = normalized * SPEED;

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
