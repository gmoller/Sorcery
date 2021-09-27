use bevy::prelude::*;

use crate::{LAYOUT_SIZE, SCALE};

const SPEED: f32 = 200.0;

pub(crate) fn move_unit(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut crate::components::Unit, &crate::components::IsMoving)>
) {
    // if a unit is moving, adjust it's transform to move closer to it's destination
    // if it has arrived at it's destination, remove the IsMoving component and set it's location

    for (entity, mut transform, mut unit, is_moving) in query.iter_mut() {

        let translation = &mut transform.translation;
        let current_point = crate::hexagons::Point::new(translation.x.into(), translation.y.into());
        let desintation_point = is_moving.destination_hex.hex_to_pixel(LAYOUT_SIZE, SCALE);
        let direction: Vec2 = calculate_direction(current_point, desintation_point);

        translation.x += direction.x * time.delta_seconds();
        translation.y += direction.y * time.delta_seconds();

        let current_point = crate::hexagons::Point::new(translation.x.into(), translation.y.into());
        let arrived_at_destination = arrived_at_destination(current_point, desintation_point, direction);
        if arrived_at_destination {
            unit.location_hex = is_moving.destination_hex;
            commands.entity(entity).remove::<crate::components::IsMoving>();
            translation.x = desintation_point.x as f32;
            translation.y = desintation_point.y as f32;

            // TODO: subtract movement_points
        }

    }

}

fn calculate_direction(current_point: crate::hexagons::Point, desintation_point: crate::hexagons::Point) -> Vec2 {
    // calculates the direction the unit needs to move in

    let x = (desintation_point.x - current_point.x) as f32;
    let y = (desintation_point.y - current_point.y) as f32;
    let vec = Vec2::new(x, y);
    //println!("vec: {:?}", vec);
    let normalized = vec.normalize();
    //println!("normalized: {:?}", normalized);
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
