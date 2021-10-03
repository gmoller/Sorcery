use std::collections;
use bevy::prelude::*;

use crate::constants::{BACKDROP_GREEN, BACKDROP_INDIGO, BACKDROP_RED, BACKLIGHT, HALF, HEX_SIZE, LAYOUT_SIZE, SCALE, UNIT_FRAME_INACTIVE, UNIT_HP_FILL, UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT, UNIT_ICON_BARBARIAN_SWORDSMEN_TRANSPARENT, UNIT_ICON_SETTLERS_TRANSPARENT};
use crate::create_bundles::create_sprite_bundle;
use crate::config::units::UnitTypes;
use crate::components::{OwnedByRace, ToBeSelectedTag, Unit, UnitBadge};
use crate::hexagons::Hex;
use crate::systems;

pub struct UnitPlugin;
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

pub fn spawn_unit(
    commands: &mut Commands,
    images: &collections::HashMap<i32, Vec<Handle<ColorMaterial>>>,
    unit_types: &UnitTypes,
    location_hex: Hex,
    unit_type_id: u16,
    race_type_id: u8,
    as_to_be_selected: bool
) {
    // spawns a unit composition entity into the ECS
    // TODO: make this a system

    let unit_type = unit_types.get_by_id(unit_type_id);
    let image_id = unit_type.image_id;
    let movement_points = unit_type.moves;

    let backdrop_material_id = match race_type_id {
        0 => panic!("Oh noes!"),
        1 => BACKDROP_INDIGO,
        2 => BACKDROP_RED,
        3 => BACKDROP_GREEN,
        4..=u8::MAX => panic!("Oh noes!")
    };

    let world_position = location_hex.hex_to_pixel(LAYOUT_SIZE, SCALE); // calculate world position from hex

    let value = images.get(&(6)); // 6: Units
    if let Some(texture_atlas) = value {

        let sprite_dimensions = Vec2::new(HEX_SIZE.0 * HALF, HEX_SIZE.1 * HALF);
        let sprite_scale = Vec3::new(SCALE.0 as f32, SCALE.1 as f32, 1.0);
        let position = Vec3::new(world_position.x as f32, world_position.y as f32, 10.0);

        let color_material_handle_backdrop = texture_atlas[backdrop_material_id as usize].clone();
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

        let unit = Unit::new(unit_type_id, location_hex, movement_points);

        // root entity holding everything together
        let entity = commands
            .spawn_bundle((Transform::from_translation(position), GlobalTransform::identity(), UnitBadge { backdrop, backlight, unit_type, hp_fill, frame }))
            .push_children(&[backdrop, backlight, unit_type, hp_fill, frame])
            .insert(unit)
            .insert(OwnedByRace::new(race_type_id))
            .id();

        if as_to_be_selected {
            commands.entity(entity).insert(ToBeSelectedTag);
        }

    }

}
