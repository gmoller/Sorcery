use bevy::prelude::*;
use bevy::input::{Input, mouse::MouseMotion};
use bevy::render::camera::Camera;

pub(crate) fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<crate::components::MainCameraTag>)>
) {
    // moves the camera if user presses arrow keys on keyboard or Left-cicks and drags the mouse

    for mut transform in camera_query.iter_mut() {
        let translate_amount = 1000.0 * time.delta_seconds();

        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= translate_amount;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += translate_amount;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += translate_amount;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= translate_amount;
        }

        for event in mouse_motion_events.iter() {
            if mouse_input.pressed(MouseButton::Left) {
                transform.translation += Vec3::new(-event.delta.x, event.delta.y, 0.0) * 5.0;
            }
        }
    }

}
