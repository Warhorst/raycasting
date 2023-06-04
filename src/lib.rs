#[derive(Copy, Clone)]
pub struct Line {
    a: (f32, f32),
    b: (f32, f32),
}

impl Line {
    fn nodes(&self) -> [(f32, f32); 2] {
        [self.a, self.b]
    }
}

impl Line {
    fn calculate_direction(&self) -> (f32, f32) {
        (self.b.0 - self.a.0, self.b.1 - self.a.1)
    }
}

pub struct Triangle {
    a: (f32, f32),
    b: (f32, f32),
    c: (f32, f32),
}

pub fn raycast(
    origin: (f32, f32),
    lines: Vec<Line>,
) -> Vec<Triangle> {
    let nodes = lines
        .iter()
        .flat_map(Line::nodes)
        .collect::<Vec<_>>();

    let mut intersection_points = vec![];

    for (x, y) in nodes {
        let direction = (x - origin.0, y - origin.1);
        let mut nearest_intersection = (x, y);
        let mut nearest_distance = calculate_distance(origin, (x, y));

        for line in &lines {
            if let Some(intersection) = get_intersection_point(origin, direction, *line) {
                let distance_to_intersection = calculate_distance(origin, intersection);

                if distance_to_intersection < nearest_distance {
                    nearest_intersection = intersection;
                    nearest_distance = distance_to_intersection
                }
            }
        }

        intersection_points.push(nearest_intersection);
    }

    intersection_points.sort_by(|p1, p2| {
        let angle_1 = calculate_angle(origin, *p1);
        let angle_2 = calculate_angle(origin, *p2);
        angle_1.total_cmp(&angle_2)
    });

    intersection_points
        .windows(2)
        .map(|nodes| Triangle {
            a: origin,
            b: nodes[0],
            c: nodes[1]
        })
        .collect()
}

// Intersection Point = P + tD
fn get_intersection_point(
    origin: (f32, f32),
    direction: (f32, f32),
    line: Line,
) -> Option<(f32, f32)> {
    let t_opt = calculate_intersection_scalar_value(origin, direction, line);
    t_opt.map(|t| (origin.0 + t * direction.0, origin.1 + t * direction.1))
}

// t = ((B - A) × (P - A)) / ((B - A) × D)
// if (B - A) × D is 0, the ray and line don't intersect
fn calculate_intersection_scalar_value(
    origin: (f32, f32),
    direction: (f32, f32),
    line: Line,
) -> Option<f32> {
    let line_direction = line.calculate_direction();
    let origin_minus_a = (origin.0 - line.a.0, origin.1 - line.a.1);
    let scalar_0 = calculate_scalar_product(line_direction, origin_minus_a);
    let scalar_1 = calculate_scalar_product(line_direction, direction);

    if scalar_1 == 0.0 {
        return None;
    }

    Some(scalar_0 / scalar_1)
}

fn calculate_scalar_product(
    p: (f32, f32),
    q: (f32, f32),
) -> f32 {
    p.0 * q.0 + p.1 * q.1
}

fn calculate_distance(
    (x1, y1): (f32, f32),
    (x2, y2): (f32, f32),
) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// TODO degree or rad? Degree for now
fn calculate_angle(
    (x1, y1): (f32, f32),
    (x2, y2): (f32, f32),
) -> f32 {
    let angle_rad = (y2 - y1).atan2(x2 - x1);
    angle_rad.to_degrees()
}