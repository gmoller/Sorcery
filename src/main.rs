use bevy::window::WindowMode;
use bevy::{prelude::*, input::system::exit_on_esc_system};
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
//use rand::{Rng, thread_rng};

use resources::ScreenSize;

use crate::config::terrain::load_terrain_types;
use crate::config::units::load_unit_types;
use crate::enums::AppState;

mod assets;
mod components;
mod config {
    pub(crate) mod terrain;
    pub(crate) mod units;
}
mod constants;
mod create_bundles;
mod enums;
mod hexagons;
mod noise;
mod resources;
mod systems {
    pub mod move_camera;
    pub mod npcs_turn;
    pub mod start_new_turn;
    pub mod player_turn;
    pub mod unit_systems;
    pub mod zoom_camera;
}
mod units;
mod world_map;
mod world_map_generator;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Sorcery".to_string(),
            //width: 1900.0,
            //height: 900.0,
            vsync: true,
            //resizable: false,
            //decorations: true,
            cursor_visible: true,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(AppState::StartNewTurnState)
        .add_startup_system(setup_system.system())
        .add_system(exit_on_esc_system.system())
        
        .add_system_set(SystemSet::on_update(AppState::StartNewTurnState)
            .with_system(systems::start_new_turn::start_new_turn_update_system.system()))
        .add_system_set(SystemSet::on_enter(AppState::StartNewTurnState).with_system(systems::start_new_turn::start_new_turn_enter_system.system()))
        .add_system_set(SystemSet::on_exit(AppState::StartNewTurnState).with_system(systems::start_new_turn::start_new_turn_exit_system.system()))

        .add_system_set(SystemSet::on_update(AppState::PlayerTurnState)
            .with_system(systems::move_camera::move_camera_system.system())
            .with_system(systems::unit_systems::check_for_unit_unselection_system.system())
            .with_system(systems::unit_systems::check_for_unit_selection_system.system().label("check_for_unit_selection_system"))
            .with_system(systems::unit_systems::select_unit_system.system().after("check_for_unit_selection_system"))
            .with_system(systems::unit_systems::check_for_unit_hover_system.system())
            .with_system(systems::unit_systems::check_for_unit_movement_system.system().label("check_for_unit_movement_system"))
            .with_system(systems::unit_systems::move_unit_system.system().after("check_for_unit_movement_system"))
            .with_system(systems::unit_systems::check_for_unit_overlap_system.system())
            .with_system(systems::player_turn::check_for_end_turn_system.system()))
        .add_system_set(SystemSet::on_enter(AppState::PlayerTurnState).with_system(systems::player_turn::player_turn_enter_system.system()))
        .add_system_set(SystemSet::on_exit(AppState::PlayerTurnState).with_system(systems::player_turn::player_turn_exit_system.system()))

        .add_system_set(SystemSet::on_update(AppState::NPCsTurnState)
            .with_system(systems::npcs_turn::give_orders_system.system().label("give_orders_system"))
            .with_system(systems::unit_systems::move_unit_system.system().after("give_orders_system"))
            .with_system(systems::unit_systems::check_for_unit_overlap_system.system())
            .with_system(systems::npcs_turn::check_for_end_turn_system.system()))
        .add_system_set(SystemSet::on_enter(AppState::NPCsTurnState).with_system(systems::npcs_turn::npcs_turn_enter_system.system()))
        .add_system_set(SystemSet::on_exit(AppState::NPCsTurnState).with_system(systems::npcs_turn::npcs_turn_exit_system.system()))

        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>
) {

    let window = windows.get_primary_mut().unwrap();
    let screen_size = ScreenSize::new(window.width(), window.height());
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(components::MainCameraTag);
    commands.spawn_bundle(UiCameraBundle::default());

    let images = assets::load_images(&asset_server, &mut materials);

    let unit_types = load_config(&mut commands);

    let world_map = world_map_generator::create_map(60, 40);
    world_map::spawn_map(&mut commands, &world_map, &images);

    // create unit on random location
    //let mut rng = thread_rng();
    //let index = rng.gen_range(0..world_map.width * world_map.height);

    let index = 60;
    spawn_unit(&mut commands, &world_map, &images, &unit_types, index, 1, true);

    let index = 61;
    spawn_unit(&mut commands, &world_map, &images, &unit_types, index, 1, false);

    let index = 62;
    spawn_unit(&mut commands, &world_map, &images, &unit_types, index, 2, false);

    // position window
    window.set_position(IVec2::new(0, 0));

    commands.insert_resource(images);
    commands.insert_resource(world_map);
    commands.insert_resource(screen_size);
}

fn load_config(commands: &mut Commands) -> config::units::UnitTypes {
    let terrain_types = load_terrain_types();
    commands.insert_resource(terrain_types.clone());
    let unit_types = load_unit_types();
    commands.insert_resource(unit_types.clone());
    
    return unit_types;
}

fn spawn_unit(commands: &mut Commands, world_map: &resources::WorldMap, images: &std::collections::HashMap<i32, Vec<Handle<ColorMaterial>>>, unit_types: &config::units::UnitTypes, index: i32, faction_id: u8, as_to_be_selected: bool) {
    let hex = world_map::convert_index_to_axial(index, world_map.width);
    // TODO: check that unit can be on the underlying tile type, and if he can't choose another hex
    units::spawn_unit(commands, images, &unit_types, hex, 2, faction_id, as_to_be_selected);
}
