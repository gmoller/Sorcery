use std::collections::HashMap;
use bevy::{prelude::*, render::camera::Camera};

use crate::constants::{UNIT_FRAME_ACTIVE, UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT};
use crate::{LAYOUT_SIZE, SCALE};
use crate::components::{MainCameraTag, ToBeSelectedTag, Unit};
use crate::units::spawn_unit_composite;

pub(crate) fn select_unit(
    mut commands: Commands,
    images: Res<HashMap<i32, Vec<Handle<ColorMaterial>>>>,
    mut unit_query: Query<(Entity, &Unit), With<ToBeSelectedTag>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<MainCameraTag>)>
) {
    // focuses camera on unit that is selected and changes unit to active

    for mut camera_transform in camera_query.iter_mut() {
        for (entity, unit) in unit_query.iter_mut() {

            // focus camera on unit:
            let world_position = unit.location_hex.hex_to_pixel(LAYOUT_SIZE, SCALE); // calculate world position from hex
            camera_transform.translation.x = world_position.x as f32;
            camera_transform.translation.y = world_position.y as f32;

            // change unit to active:
            spawn_unit_composite(&mut commands, &images, unit.location_hex, UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT, UNIT_FRAME_ACTIVE, false, true);

            // remove entity:
            commands.entity(entity).despawn_recursive();
        }
    }

}
