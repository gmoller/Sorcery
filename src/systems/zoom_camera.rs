// use bevy::prelude::*;
// use bevy::input::mouse::MouseWheel;
// use bevy::render::camera::{Camera, OrthographicProjection};

// pub(crate) fn zoom_camera(
//     mut mouse_wheel_events: EventReader<MouseWheel>,
//     windows: Res<Windows>,
//     //mut query: Query<&mut Transform, With<Camera>>
//     mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>
// ) {
//     let delta_zoom: f32 = mouse_wheel_events.iter().map(|e| e.y).sum();
//     if delta_zoom == 0.0 {
//         return;
//     }

//     //let (mut pos, mut cam) = query.single_mut().unwrap();
//     for (mut pos, mut cam) in query.iter_mut() {
//         let window = windows.get_primary().unwrap();
//         let window_size = Vec2::new(window.width(), window.height());
//         let foo = window.cursor_position().unwrap();
//         let mouse_normalized_screen_pos = (foo / window_size) * 2.0 - Vec2::ONE;
//         let mouse_world_pos = pos.translation.truncate() + mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale;

//         let zoom_speed = 0.001;
//         let min_zoom = 1.0;
//         let max_zoom = 30.0;
//         cam.scale -= zoom_speed * delta_zoom * cam.scale;
//         cam.scale = cam.scale.clamp(min_zoom, max_zoom);

//         //cam.scale = 10.0;
//         //pos.translation.z = 999.0 * pos.scale.x;

//         pos.translation = (mouse_world_pos - mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale)
//             .extend(pos.translation.z);
//     }
// }
