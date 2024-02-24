use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Camera, Camera2dBundle, Color, Commands, EventReader, MouseButton,
    OrthographicProjection, Query, Res, Transform, Window, With,
};
use bevy::window::PrimaryWindow;

pub fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: Color::rgb(0.97, 0.97, 0.97).into(),
            ..default()
        },
        ..default()
    });
}

pub fn mouse_control(
    mut evr_motion: EventReader<MouseMotion>,
    mut q_camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
) {
    if primary_window.is_empty() {
        return;
    }
    // Pan camera if left button is pressed
    if input_mouse.pressed(MouseButton::Left) && !evr_motion.is_empty() {
        let (mut transform, projection) = q_camera.single_mut();
        let window = primary_window.single();
        let window_size = Vec2::new(window.width(), window.height());
        let projection_size = projection.area.size();
        let world_units_per_device_pixel = projection_size / window_size;

        for motion in evr_motion.read() {
            let delta_world =
                motion.delta * world_units_per_device_pixel * Vec2::new(-1.0, 1.0);
            let new_position = transform.translation + delta_world.extend(0.);
            transform.translation.x = new_position.x.clamp(-1000., 1000.);
            transform.translation.y = new_position.y.clamp(-1000., 1000.);
        }
    } else {
        // this prevents a backlog of mouse movement events being processed when the mouse
        // is clicked.
        evr_motion.clear();
    }
}

pub fn scroll_zoom(
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    if !evr_scroll.is_empty() {
        let mut projection = q_camera.single_mut();

        for scroll in evr_scroll.read() {
            projection.scale = (projection.scale - scroll.y * 0.1).clamp(0.5, 2.0);
        }
    }
}
