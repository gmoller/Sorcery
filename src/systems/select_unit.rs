use std::collections::HashMap;
use bevy::{prelude::*, render::camera::Camera};

use crate::{LAYOUT_SIZE, SCALE};

pub(crate) fn select_unit(
    mut commands: Commands,
    images: Res<HashMap<i32, Vec<Handle<ColorMaterial>>>>,
    mut unit_query: Query<(Entity, &crate::components::Unit), With<crate::components::ToBeSelectedTag>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<crate::components::MainCameraTag>)>
) {
    // focuses camera on unit that is selected and changes unit to active

    for mut transform in camera_query.iter_mut() {
        for (entity, unit) in unit_query.iter_mut() {

            // focus camera on unit:
            //println!("unit location {:?}", unit.location_hex);
            let world_position = crate::hexagons::hex_to_pixel_pointy_layout(unit.location_hex, LAYOUT_SIZE, SCALE); // calculate world position from axial
            //println!("unit world position {}", world_position);
            transform.translation.x = world_position.x as f32;
            transform.translation.y = world_position.y as f32;

            // change unit to active:
            let image_id: u8 = 1; // 1: active spearmen
            crate::units::spawn_unit(&mut commands, &images, unit.location_hex, image_id, false, true);

            // remove entity:
            commands.entity(entity).despawn_recursive();
        }
    }

}
