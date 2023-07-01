use bevy::prelude::*;
use pad::{Position, p};
use rand::{Rng, thread_rng};
use TileType::*;
use crate::raycasting::{Segment, Vector};

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: usize = 30;
pub const MAP_HEIGHT: usize = 30;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_map)
        ;
    }
}

#[derive(Component)]
pub struct Tile {
    pub pos: Position,
    pub tile_type: TileType,
}

impl Tile {
    pub fn get_edges(&self) -> [Segment; 4] {
        let x = self.pos.x as f32 * TILE_SIZE;
        let y = self.pos.y as f32 * TILE_SIZE;
        let diff = TILE_SIZE / 2.0;

        [
            Segment::new(Vector::new(x - diff, y + diff), Vector::new(x + diff, y + diff)),
            Segment::new(Vector::new(x + diff, y + diff), Vector::new(x + diff, y - diff)),
            Segment::new(Vector::new(x + diff, y - diff), Vector::new(x - diff, y - diff)),
            Segment::new(Vector::new(x - diff, y - diff), Vector::new(x - diff, y + diff)),
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    fn color(&self) -> Color {
        match self {
            Floor => Color::rgba_u8(196, 164, 132, 255),
            Wall => Color::rgba_u8(101, 67, 33, 255)
        }
    }
}

fn spawn_map(
    mut commands: Commands
) {
    let mut rng = thread_rng();
    for pos in p!(0,0).iter_to(p!(MAP_WIDTH - 1, MAP_HEIGHT - 1)) {
        let tile_type = if rng.gen_bool(0.25) {
            Wall
        } else {
            Floor
        };

        commands.spawn((
            Tile {
                pos,
                tile_type,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    color: tile_type.color(),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE, 0.0)),
                ..default()
            }
        ));
    }
}