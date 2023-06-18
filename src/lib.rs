use std::collections::HashSet;
use std::ops::{Add, Div, Mul, Sub};

use ordered_float::OrderedFloat;

use crate::IntersectionStatus::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec2 {
    x: OrderedFloat<f32>,
    y: OrderedFloat<f32>,
}

impl Vec2 {
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

    pub fn diff(&self, other: Vec2) -> Vec2 {
        Vec2::new(
            other.x() - self.x(),
            other.y() - self.y(),
        )
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = f32;

    /// Cross product between two vectors
    fn mul(self, rhs: Self) -> Self::Output {
        self.x() * rhs.y() - self.y() * rhs.x()
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

#[derive(Copy, Clone)]
pub struct Segment {
    a: Vec2,
    b: Vec2,
}

impl Segment {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub fn from_coords(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self::new(Vec2::new(x0, y0), Vec2::new(x1, y1))
    }

    fn points(&self) -> [Vec2; 2] {
        [self.a, self.b]
    }

    /// Calculate the intersection between this line segment and another one.
    /// Based on this answer on stack overflow: https://stackoverflow.com/a/565282
    fn calculate_intersection(&self, other: Segment) -> IntersectionStatus {
        let p = self.a;
        let q = other.a;
        let r = self.b - self.a;
        let s = other.b - other.a;

        let r_cross_s = r * s;
        let q_minus_p = q - p;
        let q_minus_p_cross_r = q_minus_p * r;

        if r_cross_s == 0.0 && q_minus_p_cross_r == 0.0 {
            return Collinear;
        }

        if r_cross_s == 0.0 && q_minus_p_cross_r != 0.0 {
            return NotIntersecting;
        }

        let t = q_minus_p * (s / r_cross_s);
        let u = q_minus_p * (r / r_cross_s);

        if r_cross_s != 0.0 && (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            return Intersecting(p + r * t);
        }

        NotIntersecting
    }
}

#[derive(PartialEq, Debug)]
enum IntersectionStatus {
    Intersecting(Vec2),
    Collinear,
    NotIntersecting,
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub c: (f32, f32),
}

pub fn raycast(
    origin: (f32, f32),
    lines: Vec<Segment>,
) -> Vec<Triangle> {
    let origin = Vec2::new(origin.0, origin.1);

    let points = lines
        .iter()
        .flat_map(Segment::points)
        .collect::<HashSet<_>>();

    let mut intersection_points = vec![];

    for point in points {
        let origin_to_point = Segment::new(origin, point);
        let mut nearest_intersection = point;
        let mut nearest_distance = calculate_distance(origin, point);

        for line in &lines {

            if let Intersecting(intersection) = origin_to_point.calculate_intersection(*line) {
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
        let angle_0 = calculate_angle(origin, *p1);
        let angle_1 = calculate_angle(origin, *p2);
        angle_0.total_cmp(&angle_1)
    });

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
    p1: Vec2,
    p2: Vec2,
) -> f32 {
    ((p2.x() - p1.x()).powi(2) + (p2.y() - p1.y()).powi(2)).sqrt()
}

// TODO degree or rad? Degree for now
fn calculate_angle(
    p1: Vec2,
    p2: Vec2,
) -> f32 {
    let angle_rad = (p2.y() - p1.y()).atan2(p2.x() - p1.x());
    angle_rad.to_degrees()
    // angle_rad
}

#[cfg(test)]
mod tests {
    use crate::{Segment, Vec2};
    use crate::IntersectionStatus::*;

    #[test]
    fn calculate_intersection_works() {
        let line = Segment::from_coords(0.0, 0.0, 5.0, 0.0);

        [
            (
                Segment::from_coords(3.0, 3.0, 3.0, -3.0),
                Intersecting(Vec2::new(3.0, 0.0))
            ),
            (
                Segment::from_coords(0.0, 3.0, 0.0, -3.0),
                Intersecting(Vec2::new(0.0, 0.0))
            ),
            (
                Segment::from_coords(5.0, 3.0, 5.0, -3.0),
                Intersecting(Vec2::new(5.0, 0.0))
            ),
            (
                Segment::from_coords(0.0, 0.0, 3.0, 0.0),
                Collinear
            )
        ].into_iter().for_each(|(l, intersection)| assert_eq!(line.calculate_intersection(l), intersection))
    }
}