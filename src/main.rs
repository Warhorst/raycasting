mod map;
mod line_of_sight;
mod mouse_cursor;
mod raycasting;

use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::line_of_sight::LineOfSightPlugin;
use crate::map::{MAP_HEIGHT, MAP_WIDTH, MapPlugin};
use crate::mouse_cursor::MouseCursorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800.0, 600.0).into(),
                        title: "raycasting".to_string(),
                        resizable: false,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_event::<UpdateLos>()
        .add_plugin(MapPlugin)
        .add_plugin(LineOfSightPlugin)
        .add_plugin(MouseCursorPlugin)
        .add_startup_system(spawn_camera)
        .run()
}

pub struct UpdateLos;

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 2.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (MAP_WIDTH as f32 / 2.0) * 32.0,
                (MAP_HEIGHT as f32 / 2.0) * 32.0,
                1000.0
            )),
            ..default()
        }
    );
}