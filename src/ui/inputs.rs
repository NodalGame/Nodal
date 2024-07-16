pub mod inputs {
    use bevy::{
        math::Vec2,
        prelude::{Query, Res, ResMut, With},
        render::camera::Camera,
        sprite::Sprite,
        transform::components::{GlobalTransform, Transform},
        window::{PrimaryWindow, Window},
    };

    use crate::{
        is_mouse_over_sprite, Hoverable, MainCamera, MousePosition, SCALE_NODE_DEFAULT,
        SCALE_NODE_HOVERED,
    };

    /// A system for tracking mouse position in the window.
    pub fn mouse_position_system(
        mut mouse_position: ResMut<MousePosition>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // Get camera info and transform, assuming exacly 1 camera entity
        let (camera, camera_transform) = q_camera.single();

        // Only one primary window, so get it from query
        let window = q_window.single();

        // Check if cursor inside window and get its position, convert to world coords, discard Z
        mouse_position.position = get_cursor_world_position(window, camera, camera_transform);
    }

    fn get_cursor_world_position(
        window: &Window,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Vec2 {
        return window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
            .unwrap_or(Vec2::MIN);
    }

    /// A system for updating hovered/unhovered sprites.
    pub fn hover_system(
        mouse_position: Res<MousePosition>,
        mut q_hoverable: Query<(&Hoverable, &mut Transform, &Sprite)>,
    ) {
        for (_hoverable, mut transform, sprite) in q_hoverable.iter_mut() {
            if is_mouse_over_sprite(sprite, *transform, mouse_position.position) {
                transform.scale = SCALE_NODE_HOVERED;
            } else {
                transform.scale = SCALE_NODE_DEFAULT;
            }
        }
    }
}
