use bevy::prelude::*;
use bevy::input::{Input, mouse::MouseMotion};
use bevy::render::camera::Camera;

pub(crate) fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<crate::components::MainCameraTag>)>
) {
    // moves the camera if user presses arrow keys on keyboard or Left-cicks and drags the mouse

    let mut camera_transform = camera_query.single_mut().unwrap();

    let translate_amount = 1000.0 * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::Left) {
        camera_transform.translation.x -= translate_amount;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        camera_transform.translation.x += translate_amount;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        camera_transform.translation.y += translate_amount;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        camera_transform.translation.y -= translate_amount;
    }

    for event in mouse_motion_events.iter() {
        if mouse_input.pressed(MouseButton::Left) {
            camera_transform.translation += Vec3::new(-event.delta.x, event.delta.y, 0.0) * 5.0;
        }
    }

    camera_transform.translation.x = camera_transform.translation.x.clamp(950.0, 15360.0 - 950.0);
    camera_transform.translation.y = camera_transform.translation.y.clamp(-7500.0, -450.0);

    //println!("Camera coords: {}/{}", camera_transform.translation.x, camera_transform.translation.y);

}
