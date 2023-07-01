use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(super) struct MouseCursorPlugin;

impl Plugin for MouseCursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorCoordinates>()
            .add_system(update_cursor_position)
        ;
    }
}

#[derive(Default, Deref, DerefMut, Resource)]
pub struct CursorCoordinates(Vec2);

fn update_cursor_position(
    mut cursor_position: ResMut<CursorCoordinates>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = cameras.single();
    let window = windows.single();

    if let Some(position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let ndc = (position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        if world_pos != **cursor_position {
            **cursor_position = world_pos
        }
    }
}