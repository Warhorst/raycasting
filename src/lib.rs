mod intersection;

use std::collections::HashSet;
use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector {
    x: OrderedFloat<f32>,
    y: OrderedFloat<f32>,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: OrderedFloat::from(x),
            y: OrderedFloat::from(y),
        }
    }

    pub fn tuple(&self) -> (f32, f32) {
        (self.x.0, self.y.0)
    }

    pub fn x(&self) -> f32 {
        self.x.0
    }

    pub fn y(&self) -> f32 {
        self.y.0
    }

    pub fn cross_product(&self, other: Vector) -> f32 {
        self.x() * other.y() - self.y() * other.x()
    }

    pub fn diff(&self, other: Vector) -> Vector {
        Vector::new(
            other.x() - self.x(),
            other.y() - self.y(),
        )
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    a: Vector,
    b: Vector,
}

impl Line {
    pub fn new(a: Vector, b: Vector) -> Self {
        Self { a, b }
    }

    pub fn from_coords(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self::new(Vector::new(x0, y0), Vector::new(x1, y1))
    }

    fn points(&self) -> [Vector; 2] {
        [self.a, self.b]
    }

    fn calculate_intersection(&self, other: Line) -> Option<Vector> {
        let slope_0 = self.calculate_slope();
        let slope_1 = other.calculate_slope();

        if slope_0 == slope_1 {
            return None;
        }

        let intercept_0 = self.a.y() - slope_0 * self.a.x();
        let intercept_1 = other.a.y() - slope_1 * other.a.x();

        let x = (intercept_1 - intercept_0) / (slope_0 - slope_1);
        let y = slope_0 * x + intercept_0;

        if x < self.a.x().min(self.b.x()) || x > self.a.x().max(self.b.x()) || y < self.a.y().min(self.b.y()) || y > self.a.y().max(self.b.y()) ||
            x < other.a.x().min(other.b.x()) || x > other.a.x().max(other.b.x()) || y < other.a.y().min(other.b.y()) || y > other.a.y().max(other.b.y()) {
            return None;
        }

        Some(Vector::new(x, y))
    }

    fn calculate_intersection_(&self, other: Line) -> Option<Vector> {
        let p0_x = self.a.x();
        let p0_y = self.a.y();
        let p1_x = self.b.x();
        let p1_y = self.b.y();
        let p2_x = other.a.x();
        let p2_y = other.a.y();
        let p3_x = other.b.x();
        let p3_y = other.b.y();

        let s1_x = p1_x - p0_x;
        let s1_y = p1_y - p0_y;
        let s2_x = p3_x - p2_x;
        let s2_y = p3_y - p2_y;

        let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = (s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);

        if (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t) {
            Some(Vector::new(
                p0_x + (t * s1_x),
                p0_y + (t * s1_y)
            ))
        } else {
            None
        }
    }

    fn calculate_slope(&self) -> f32 {
        (self.b.y() - self.a.y()) / (self.b.x() - self.a.x())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub c: (f32, f32),
}

pub fn raycast(
    origin: (f32, f32),
    lines: Vec<Line>,
) -> Vec<Triangle> {
    let origin = Vector::new(origin.0, origin.1);

    let points = lines
        .iter()
        .flat_map(Line::points)
        .collect::<HashSet<_>>();

    let mut intersection_points = vec![];

    for point in points {
        println!("Point: {:?}", point);
        let origin_to_point = Line::new(origin, point);
        let mut nearest_intersection = point;
        let mut nearest_distance = calculate_distance(origin, point);

        for line in &lines {
            if let Some(intersection) = origin_to_point.calculate_intersection(*line) {
                let distance_to_intersection = calculate_distance(origin, intersection);

                if distance_to_intersection < nearest_distance {
                    println!("Last nearest intersection: {:?}", nearest_intersection);
                    println!("New nearest intersection: {:?}", intersection);
                    nearest_intersection = intersection;
                    nearest_distance = distance_to_intersection
                }
            }
        }

        intersection_points.push(nearest_intersection);
    }

    intersection_points.sort_by(|p1, p2| {
        let angle_0 = calculate_angle(origin, *p1);
        let angle_1 = calculate_angle(origin, *p2);
        angle_0.total_cmp(&angle_1)
    });

    println!("{:?}", intersection_points);

    let mut triangles = intersection_points
        .windows(2)
        .map(|nodes| Triangle {
            a: origin.tuple(),
            b: nodes[0].tuple(),
            c: nodes[1].tuple(),
        })
        .collect::<Vec<_>>();

    triangles.push(Triangle {
        a: origin.tuple(),
        b: intersection_points.first().unwrap().tuple(),
        c: intersection_points.last().unwrap().tuple(),
    });

    triangles
}

fn calculate_distance(
    p1: Vector,
    p2: Vector,
) -> f32 {
    ((p2.x() - p1.x()).powi(2) + (p2.y() - p1.y()).powi(2)).sqrt()
}

// TODO degree or rad? Degree for now
fn calculate_angle(
    p1: Vector,
    p2: Vector,
) -> f32 {
    let angle_rad = (p2.y() - p1.y()).atan2(p2.x() - p1.x());
    angle_rad.to_degrees()
    // angle_rad
}

#[cfg(test)]
mod tests {
    use crate::{Line, Vector};

    #[test]
    fn calculate_intersection_works() {
        let line = Line::from_coords(0.0, 0.0, 5.0, 0.0);

        [
            (
                Line::from_coords(3.0, 3.0, 3.0, -3.0),
                Some(Vector::new(3.0, 0.0))
            ),
            (
                Line::from_coords(0.0, 3.0, 0.0, -3.0),
                Some(Vector::new(0.0, 0.0))
            ),
            (
                Line::from_coords(5.0, 3.0, 5.0, -3.0),
                Some(Vector::new(5.0, 0.0))
            ),
            (
                Line::from_coords(0.0, 0.0, 3.0, 0.0),
                Some(Vector::new(5.0, 0.0))
            )
        ].into_iter().for_each(|(l, intersection)| assert_eq!(line.calculate_intersection_(l), intersection))
    }
}