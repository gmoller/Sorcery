use std::collections;
use bevy::prelude::*;

use crate::constants::{BACKDROP_INDIGO, BACKLIGHT, HALF, HEX_SIZE, LAYOUT_SIZE, SCALE, UNIT_FRAME_INACTIVE, UNIT_HP_FILL, UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT, UNIT_ICON_BARBARIAN_SWORDSMEN_TRANSPARENT, UNIT_ICON_SETTLERS_TRANSPARENT};
use crate::create_bundles::create_sprite_bundle;
use crate::components::{ToBeSelectedTag, Unit, UnitBadge};
use crate::hexagons::Hex;
use crate::systems;

pub(crate) struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(systems::unit_systems::select_unit_system.system())
        .add_system(systems::unit_systems::check_for_unit_movement_system.system())
        .add_system(systems::unit_systems::check_for_unit_selection_system.system())
        .add_system(systems::unit_systems::check_for_unit_unselection_system.system())
        .add_system(systems::unit_systems::check_for_unit_hover_system.system())
        .add_system(systems::unit_systems::move_unit_system.system());
    }
}

pub(crate) fn spawn_unit(
    commands: &mut Commands,
    images: &collections::HashMap<i32, Vec<Handle<ColorMaterial>>>,
    location_hex: Hex,
    unit_type_id: u8,
    as_to_be_selected: bool
) {
    // spawns a unit composition entity into the ECS
    // TODO: make this a system

    let image_id = match unit_type_id {
        0 => 0,
        1 => UNIT_ICON_SETTLERS_TRANSPARENT,
        2 => UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT,
        3 => UNIT_ICON_BARBARIAN_SWORDSMEN_TRANSPARENT,
        4..=u8::MAX => 4
    };

    let world_position = location_hex.hex_to_pixel(LAYOUT_SIZE, SCALE); // calculate world position from hex

    let value = images.get(&(6)); // 6: Units
    if let Some(texture_atlas) = value {

        let sprite_dimensions = Vec2::new(HEX_SIZE.0 * HALF, HEX_SIZE.1 * HALF);
        let sprite_scale = Vec3::new(SCALE.0 as f32, SCALE.1 as f32, 1.0);
        let position = Vec3::new(world_position.x as f32, world_position.y as f32, 10.0);

        let color_material_handle_backdrop = texture_atlas[BACKDROP_INDIGO as usize].clone();
        //let color = &mut materials.get_mut(&color_material_handle_backdrop).unwrap().color;
        //color.set_r(0.0); //color.set_g(1.0); //color.set_b(0.0);
        let color_material_handle_backlight = texture_atlas[BACKLIGHT as usize].clone();
        let color_material_handle_unit_type = texture_atlas[image_id as usize].clone();
        let color_material_handle_unit_hp_fill = texture_atlas[UNIT_HP_FILL as usize].clone();
        let color_material_handle_unit_frame = texture_atlas[UNIT_FRAME_INACTIVE as usize].clone();

        let backdrop_bundle = create_sprite_bundle(Vec2::new(HEX_SIZE.0 * 0.41, HEX_SIZE.1 * 0.41), Vec3::new(0.0, 0.0, -3.0), sprite_scale, color_material_handle_backdrop);
        let backdrop = commands.spawn_bundle(backdrop_bundle).id();

        let backlight_bundle = create_sprite_bundle(Vec2::new(HEX_SIZE.0 * 0.41, HEX_SIZE.1 * 0.41), Vec3::new(0.0, 0.0, -2.0), sprite_scale, color_material_handle_backlight);
        let backlight = commands.spawn_bundle(backlight_bundle).id();

        let unit_type_bundle = create_sprite_bundle(Vec2::new(HEX_SIZE.0 * 0.28, HEX_SIZE.1 * 0.25), Vec3::new(0.0, HEX_SIZE.1 * 0.023, -1.0), sprite_scale, color_material_handle_unit_type);
        let unit_type = commands.spawn_bundle(unit_type_bundle).id();

        let hp_fill_bundle = create_sprite_bundle(Vec2::new(HEX_SIZE.0 * 0.31, HEX_SIZE.1 * 0.05), Vec3::new(0.0, -HEX_SIZE.1 * 0.15, -1.0), sprite_scale, color_material_handle_unit_hp_fill);
        let hp_fill = commands.spawn_bundle(hp_fill_bundle).id();

        let frame_bundle = create_sprite_bundle(sprite_dimensions, Vec3::default(), sprite_scale, color_material_handle_unit_frame);
        let frame = commands.spawn_bundle(frame_bundle).id();

        // root entity holding everything together
        if as_to_be_selected {
            commands
                .spawn_bundle((Transform::from_translation(position), GlobalTransform::identity(), UnitBadge { backdrop, backlight, unit_type, hp_fill, frame }))
                .push_children(&[backdrop, backlight, unit_type, hp_fill, frame])
                .insert(Unit::new(1, location_hex))
                .insert(ToBeSelectedTag);

                return;
        }

        commands
            .spawn_bundle((Transform::from_translation(position), GlobalTransform::identity(), UnitBadge { backdrop, backlight, unit_type, hp_fill, frame }))
            .push_children(&[backdrop, backlight, unit_type, hp_fill, frame])
            .insert(Unit::new(1, location_hex));

    }

}
