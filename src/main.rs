use bevy::{prelude::*, input::system::exit_on_esc_system};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use rand::{Rng, thread_rng};

mod assets;
mod components;
mod create_bundles;
mod hexagons;
mod noise;
mod resources;
mod systems {
    pub(crate) mod check_for_unit_movement;
    pub(crate) mod move_camera;
    pub(crate) mod move_unit;
    pub(crate) mod select_unit;
    pub(crate) mod zoom_camera;
}
mod units;
mod world_map;
mod world_map_generator;

const SCALE: (f32, f32) = (1.0, 1.0);
const HALF: f32 = 0.5;
const THREE_QUARTERS: f32 = 0.75;
//const HEX_SIZE: (f32, f32) = (32.0, 48.0);
//const HEX_SIZE: (f32, f32) = (64.0, 96.0);
//const HEX_SIZE: (f32, f32) = (128.0, 192.0);
const HEX_SIZE: (f32, f32) = (256.0, 384.0);
const HEX_OFFSET_Y: f32 = HEX_SIZE.1 / 6.0;
const LAYOUT_SIZE: (f32, f32) = (HEX_SIZE.0 * 0.57421875, HEX_SIZE.1 * 0.3125);
const CROSSHAIR: &str = "images/crosshair.png";

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK)) // PINK
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Sorcery".to_string(),
            width: 1900.0, // 1600
            height: 900.0,
            vsync: true,
            resizable: false,
            decorations: true, // show title bar
            cursor_visible: true,
            //mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_system.system())
        .add_system(exit_on_esc_system.system())
        .add_system(systems::move_camera::move_camera.system())
        .add_system(systems::select_unit::select_unit.system())
        .add_system(systems::check_for_unit_movement::check_for_unit_movement.system())
        .add_system(systems::move_unit::move_unit.system())
        //.add_system(zoom_camera.system())
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    //let screen_size = ScreenSize::new(window.width(), window.height());
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(components::MainCameraTag);
    commands.spawn_bundle(UiCameraBundle::default());

    let images = assets::load_images(&asset_server, &mut materials);

    let world_map = world_map_generator::create_map(60, 40);
    world_map::spawn_map(&mut commands, &world_map, &images);

    // create unit on random location
    let mut rng = thread_rng();
    let index = rng.gen_range(0..world_map.width * world_map.height);
    //let index = 0;
    let axial = world_map::convert_index_to_axial(index.into(), world_map.width);
    // TODO: check that unit can be on the underlying tile type, and if he can't choose another hex
    let image_id = 3; // 3: inactive spearmen
    units::spawn_unit(&mut commands, &images, axial, image_id, true, false);

    draw_crosshair(&asset_server, &mut materials, &mut commands);

    // position window
    window.set_position(IVec2::new(0, 0));

    commands.insert_resource(images);
}

fn draw_crosshair(asset_server: &Res<AssetServer>, materials: &mut ResMut<Assets<ColorMaterial>>, commands: &mut Commands) {
    let texture_handle = asset_server.load(CROSSHAIR);
    let material = materials.add(texture_handle.into());
    let sprite_dimensions = Vec2::new(50.0, 50.0);
    let sprite_scale = Vec3::new(1.0, 1.0, 1.0);
    let position = Vec3::new(0.0, 0.0, 10.0);
    let bundle = create_bundles::create_sprite_bundle(sprite_dimensions, position, sprite_scale, material);
    commands.spawn_bundle(bundle);
}