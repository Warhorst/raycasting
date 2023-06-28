use std::collections::HashSet;
use std::f32::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

use ordered_float::OrderedFloat;

use crate::IntersectionStatus::*;

// TODO I need Ray segment intersection, not segment segment intersection
// TODO Idea for optimization
//  1. send a ray to the target point
//  2. if it hits the point, send another ray from that point in the same direction. If not, dont send another ray

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

    pub fn cross_product(&self, other: Vec2) -> f32 {
        self.x() * other.y() - self.y() * other.x()
    }

    pub fn dot_product(&self, other: Vec2) -> f32 {
        self.x() * other.x() + self.y() * other.y()
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
            y: self.y / rhs,
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
    ///
    /// Basically, there are 4 cases
    ///
    /// 1. The segments are collinear (r × s = 0 and (q − p) × r = 0)
    /// If the segments are collinear, tow sub-cases could happen
    ///
    /// 1.1 The segments intersect and the intersection is another segment
    /// This is checked by calculating two values
    /// t0 = (q − p) · r / (r · r)
    /// t1 = t0 + s · r / (r · r)
    ///
    /// and check if they intersect with the interval [0,1]. If true, the segments are collinear and intersecting
    ///
    /// 1.2 The segments don't intersect
    /// If the check from 1.1 is false, the segments are collinear but don't intersect
    ///
    /// 2. The segments are parallel but don't intersect (r × s = 0 and (q − p) × r ≠ 0)
    ///
    /// 3. The segments are intersecting (r × s ≠ 0 and 0 ≤ t ≤ 1 and 0 ≤ u ≤ 1)
    /// t = (q − p) × s / (r × s)
    /// u = (p − q) × r / (s × r)
    ///
    /// Then the intersection is p + t r = q + u s
    ///
    /// 4. The segments are neither collinear nor parallel. They just dont intersect
    fn calculate_intersection(&self, other: Segment) -> IntersectionStatus {
        let p = self.a;
        let q = other.a;
        let r = self.b - self.a;
        let s = other.b - other.a;

        let r_cross_s = r.cross_product(s);
        let q_minus_p = q - p;
        let q_minus_p_cross_r = q_minus_p.cross_product(r);

        if r_cross_s == 0.0 && q_minus_p_cross_r == 0.0 {
            let t0 = q_minus_p.dot_product(r) / (r.dot_product(r));
            let t1 = t0 + ((s.dot_product(r)) / (r.dot_product(r)));

            let interval = 0.0..=1.0;

            if interval.contains(&t0) || interval.contains(&t1) || (t0 <= 0.0 && t1 >= 1.0) {
                return CollinearIntersecting;
            } else {
                return CollinearNotIntersecting;
            }
        }

        if r_cross_s == 0.0 && q_minus_p_cross_r != 0.0 {
            return NotIntersecting;
        }

        let t = q_minus_p.cross_product(s / r_cross_s);
        let u = q_minus_p.cross_product(r / r_cross_s);

        if r_cross_s != 0.0 && (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            return Intersecting(p + r * t);
        }

        NotIntersecting
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec2,
    direction: Vec2,
}

impl Ray {
    fn new(
        origin: Vec2,
        direction: Vec2
    ) -> Self {
        Ray {
            origin,
            direction
        }
    }

    fn calculate_intersection(&self, segment: Segment) -> IntersectionStatus {
        let p = self.origin;
        let q = segment.a;
        let r = self.direction;
        let s = segment.b - segment.a;

        let r_cross_s = r.cross_product(s);
        let q_minus_p = q - p;
        let q_minus_p_cross_r = q_minus_p.cross_product(r);

        if r_cross_s == 0.0 && q_minus_p_cross_r == 0.0 {
            let t0 = q_minus_p.dot_product(r) / (r.dot_product(r));
            let t1 = t0 + ((s.dot_product(r)) / (r.dot_product(r)));

            let interval = 0.0..=1.0;

            if interval.contains(&t0) || interval.contains(&t1) || (t0 <= 0.0 && t1 >= 1.0) {
                return CollinearIntersecting;
            } else {
                return CollinearNotIntersecting;
            }
        }

        if r_cross_s == 0.0 && q_minus_p_cross_r != 0.0 {
            return NotIntersecting;
        }

        let t = q_minus_p.cross_product(s / r_cross_s);
        let u = q_minus_p.cross_product(r / r_cross_s);

        if r_cross_s != 0.0 && t >= 0.0 && (0.0..=1.0).contains(&u) {
            return Intersecting(p + r * t);
        }

        NotIntersecting
    }

    fn rotate(&self, radians: f32) -> Self {
        let rotated_direction = Vec2::new(
            self.direction.x() * radians.cos() - self.direction.y() * radians.sin(),
            self.direction.x() * radians.sin() + self.direction.y() * radians.cos()
        );

        Ray {
            origin: self.origin,
            direction: rotated_direction
        }
    }
}

#[derive(PartialEq, Debug)]
enum IntersectionStatus {
    Intersecting(Vec2),
    CollinearIntersecting,
    CollinearNotIntersecting,
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
    segments: Vec<Segment>,
) -> Vec<Triangle> {
    let origin = Vec2::new(origin.0, origin.1);

    let points = segments
        .iter()
        .flat_map(Segment::points)
        .collect::<HashSet<_>>();

    let mut intersection_points = Vec::with_capacity(points.len());

    for point in points {
        let direction = point - origin;
        let origin_to_point = Ray::new(origin, direction);

        let rays = [origin_to_point, origin_to_point.rotate(0.01), origin_to_point.rotate(-0.01)];

        println!("Begin");
        for ray in rays {
            println!("Ray: {:?}", ray);

            let mut nearest_intersection = None;
            let mut nearest_distance = f32::MAX;

            for segment in &segments {
                // TODO Collinear intersecting is a special case
                if let Intersecting(intersection) = ray.calculate_intersection(*segment) {
                    let distance_to_intersection = calculate_distance(origin, intersection);

                    if distance_to_intersection < nearest_distance {
                        nearest_intersection = Some(intersection);
                        nearest_distance = distance_to_intersection
                    }
                }
            }

            if let Some(intersection) = nearest_intersection {
                intersection_points.push(intersection);
            }
        }
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

pub fn raycast_only_points(
    origin: (f32, f32),
    segments: Vec<Segment>,
) -> Vec<(f32, f32)> {
    let origin = Vec2::new(origin.0, origin.1);

    let points = segments
        .iter()
        .flat_map(Segment::points)
        .collect::<HashSet<_>>();

    let mut intersection_points = Vec::with_capacity(points.len());

    for point in points {
        let direction = point - origin;
        let origin_to_point = Ray::new(origin, direction);

        let rays = [origin_to_point, origin_to_point.rotate(0.01), origin_to_point.rotate(-0.01)];

        println!("Begin");
        for ray in rays {
            println!("Ray: {:?}", ray);

            let mut nearest_intersection = None;
            let mut nearest_distance = f32::MAX;

            for segment in &segments {
                // TODO Collinear intersecting is a special case
                if let Intersecting(intersection) = ray.calculate_intersection(*segment) {
                    let distance_to_intersection = calculate_distance(origin, intersection);

                    if distance_to_intersection < nearest_distance {
                        nearest_intersection = Some(intersection);
                        nearest_distance = distance_to_intersection
                    }
                }
            }

            if let Some(intersection) = nearest_intersection {
                intersection_points.push(intersection);
            }
        }
    }

    intersection_points.into_iter().map(|vec2| (vec2.x(), vec2.y())).collect()
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
    use crate::{Ray, Segment, Vec2};
    use crate::IntersectionStatus::*;

    #[test]
    fn segment_segment_intersection_works() {
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
                Segment::from_coords(0.0, -1.0, 5.0, 1.0),
                Intersecting(Vec2::new(2.5, 0.0))
            ),
            (
                Segment::from_coords(0.0, 0.0, 3.0, 0.0),
                CollinearIntersecting
            ),
            (
                Segment::from_coords(3.0, 0.0, 0.0, 0.0),
                CollinearIntersecting
            ),
            (
                Segment::from_coords(6.0, 0.0, 10.0, 0.0),
                CollinearNotIntersecting
            ),
            (
                Segment::from_coords(10.0, 0.0, 6.0, 0.0),
                CollinearNotIntersecting
            ),
            (
                Segment::from_coords(0.0, 1.0, 5.0, 1.0),
                NotIntersecting
            )
        ].into_iter().for_each(|(l, intersection)| assert_eq!(line.calculate_intersection(l), intersection))
    }

    #[test]
    fn ray_segment_intersection_works() {
        let ray = Ray {
            origin: Vec2::new(0.0, 0.0),
            direction: Vec2::new(5.0, 0.0),
        };

        [
            (
                Segment::from_coords(7.0, 3.0, 7.0, -3.0),
                Intersecting(Vec2::new(7.0, 0.0))
            ),
            (
                Segment::from_coords(7000.0, 3.0, 7000.0, -3.0),
                Intersecting(Vec2::new(7000.0, 0.0))
            ),
            (
                Segment::from_coords(-1.0, 3.0, -1.0, -3.0),
                NotIntersecting
            ),
            (
                Segment::from_coords(0.0, 3.0, -3.0, 0.0),
                NotIntersecting
            ),
            (
                Segment::from_coords(0.0, 3.0, 5.0, 3.0),
                NotIntersecting
            ),
            (
                Segment::from_coords(-5.0, 0.0, -3.0, 0.0),
                CollinearNotIntersecting
            ),
            (
                Segment::from_coords(0.0, 0.0, 3.0, 0.0),
                CollinearIntersecting
            )
        ].into_iter().for_each(|(segment, intersection)| assert_eq!(ray.calculate_intersection(segment), intersection))
    }
}