use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;
use crate::map::{MAP_HEIGHT, MAP_WIDTH, Tile, TILE_SIZE, TileType};
use crate::mouse_cursor::CursorCoordinates;
use crate::raycasting::{raycast, Segment, Triangle};

pub struct LineOfSightPlugin;

impl Plugin for LineOfSightPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LineOfSight(vec![]))
            .insert_resource(IntersectionPoints(vec![]))
            .add_systems((
                update_los,
                spawn_los_triangles,
                spawn_intersection_lines
            ))
        ;
    }
}

#[derive(Resource)]
pub struct LineOfSight(Vec<Triangle>);

#[derive(Resource)]
pub struct IntersectionPoints(Vec<((f32, f32), (f32, f32))>);

#[derive(Component)]
struct LosTriangle;

#[derive(Component)]
struct IntersectionLine;

fn update_los(
    mouse_coordinates: Res<CursorCoordinates>,
    mut line_of_sight: ResMut<LineOfSight>,
    mut intersection_points: ResMut<IntersectionPoints>,
    query: Query<&Tile>,
) {
    if !mouse_coordinates.is_changed() {
        return;
    }

    let origin = (mouse_coordinates.x, mouse_coordinates.y);
    let mut lines = query
        .iter()
        .filter(|tile| tile.tile_type == TileType::Wall)
        .flat_map(|tile| tile.get_edges())
        .collect::<Vec<_>>();


    lines.extend([
        Segment::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(MAP_WIDTH as f32 * TILE_SIZE, 0.0),
        ),
        Segment::new(
            Vec2::new(MAP_WIDTH as f32 * TILE_SIZE, 0.0),
            Vec2::new(MAP_WIDTH as f32 * TILE_SIZE, MAP_HEIGHT as f32 * TILE_SIZE),
        ),
        Segment::new(
            Vec2::new(MAP_WIDTH as f32 * TILE_SIZE, MAP_HEIGHT as f32 * TILE_SIZE),
            Vec2::new(0.0, MAP_HEIGHT as f32 * TILE_SIZE),
        ),
        Segment::new(
            Vec2::new(0.0, MAP_HEIGHT as f32 * TILE_SIZE),
            Vec2::new(0.0, 0.0),
        ),
    ]);

    let origin = Vec2::new(origin.0, origin.1);
    let triangles = raycast(origin, lines.clone());
    *line_of_sight = LineOfSight(triangles);
    // *intersection_points = IntersectionPoints(calculate_intersection_points(origin, lines).into_iter().map(|point| ((origin.x, origin.y), (point.x, point.y))).collect())
}

fn spawn_los_triangles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    line_of_sight: Res<LineOfSight>,
    los_triangles: Query<Entity, With<LosTriangle>>,
) {
    if !line_of_sight.is_changed() {
        return;
    }

    for e in &los_triangles {
        commands.entity(e).despawn();
    }

    let color = Color::from([1.0, 1.0, 1.0, 0.5]);

    for triangle in line_of_sight.0.iter() {
        commands.spawn((
            LosTriangle,
            MaterialMesh2dBundle {
                mesh: meshes.add(create_triangle(*triangle)).into(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                material: materials.add(ColorMaterial::from(color)),
                ..Default::default()
            }));
    }
}

fn create_triangle(triangle: Triangle) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[triangle.a.0, triangle.a.1, 0.0], [triangle.b.0, triangle.b.1, 0.0], [triangle.c.0, triangle.c.1, 0.0]],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1.0, 1.0, 1.0, 1.0]; 3]);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

fn spawn_intersection_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    intersection_points: Res<IntersectionPoints>,
    intersection_lines: Query<Entity, With<IntersectionLine>>
) {
    if !intersection_points.is_changed() {
        return;
    }

    for e in &intersection_lines {
        commands.entity(e).despawn();
    }

    let color = Color::from([1.0, 0.0, 0.0, 1.0]);

    for (origin, point) in intersection_points.0.iter() {
        commands.spawn((
            LosTriangle,
            MaterialMesh2dBundle {
                mesh: meshes.add(create_line(*origin, *point)).into(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
                material: materials.add(ColorMaterial::from(color)),
                ..Default::default()
            }));
    }
}

fn create_line(origin: (f32,  f32), point: (f32, f32)) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[origin.0, origin.1, 0.0], [point.0, point.1, 0.0]],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1.0, 1.0, 1.0, 1.0]; 2]);
    mesh.set_indices(Some(Indices::U32(vec![0, 1])));
    mesh
}