use std::ops::{Add, Div, Mul, Sub};

use crate::IntersectionStatus::*;

// TODO I need Ray segment intersection, not segment segment intersection
// TODO Idea for optimization
//  1. send a ray to the target point
//  2. if it hits the point, send another ray from that point in the same direction. If not, dont send another ray

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn cross_product(&self, other: Vector) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn dot_product(&self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Segment {
    a: Vector,
    b: Vector,
}

impl Segment {
    pub fn new(a: Vector, b: Vector) -> Self {
        Self { a, b }
    }

    pub fn from_coords(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self::new(Vector::new(x0, y0), Vector::new(x1, y1))
    }

    fn points(&self) -> [Vector; 2] {
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
    origin: Vector,
    direction: Vector,
}

impl Ray {
    fn new(
        origin: Vector,
        direction: Vector
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
        let rotated_direction = Vector::new(
            self.direction.x * radians.cos() - self.direction.y * radians.sin(),
            self.direction.x * radians.sin() + self.direction.y * radians.cos()
        );

        Ray {
            origin: self.origin,
            direction: rotated_direction
        }
    }
}

#[derive(PartialEq, Debug)]
enum IntersectionStatus {
    Intersecting(Vector),
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
    origin: Vector,
    segments: Vec<Segment>,
) -> Vec<Triangle> {
    let intersection_points = calculate_intersection_points(origin, segments);

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

/// Return every intersection point of rays from origin to every point of the segments and the segments itself.
///The intersection points are ordered by angle.
pub fn calculate_intersection_points(
    origin: Vector,
    segments: Vec<Segment>,
) -> Vec<Vector> {
    let mut points = segments
        .iter()
        .flat_map(Segment::points)
        .collect::<Vec<_>>();

    points.sort_by(|p1, p2| {
        let angle_0 = calculate_angle(origin, *p1);
        let angle_1 = calculate_angle(origin, *p2);
        angle_0.total_cmp(&angle_1)
    });
    points.dedup();

    let mut intersection_points = Vec::with_capacity(points.len());

    for point in points {
        let direction = point - origin;
        let origin_to_point = Ray::new(origin, direction);

        let rays = [origin_to_point.rotate(-0.01), origin_to_point, origin_to_point.rotate(0.01)];

        for ray in rays {
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

    intersection_points
}

fn calculate_distance(
    p1: Vector,
    p2: Vector,
) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

fn calculate_angle(
    p1: Vector,
    p2: Vector,
) -> f32 {
    let angle_rad = (p2.y - p1.y).atan2(p2.x - p1.x);
    angle_rad.to_degrees()
}

#[cfg(test)]
mod tests {
    use crate::{Ray, Segment, Vector};
    use crate::IntersectionStatus::*;

    #[test]
    fn segment_segment_intersection_works() {
        let line = Segment::from_coords(0.0, 0.0, 5.0, 0.0);

        [
            (
                Segment::from_coords(3.0, 3.0, 3.0, -3.0),
                Intersecting(Vector::new(3.0, 0.0))
            ),
            (
                Segment::from_coords(0.0, 3.0, 0.0, -3.0),
                Intersecting(Vector::new(0.0, 0.0))
            ),
            (
                Segment::from_coords(5.0, 3.0, 5.0, -3.0),
                Intersecting(Vector::new(5.0, 0.0))
            ),
            (
                Segment::from_coords(0.0, -1.0, 5.0, 1.0),
                Intersecting(Vector::new(2.5, 0.0))
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
            origin: Vector::new(0.0, 0.0),
            direction: Vector::new(5.0, 0.0),
        };

        [
            (
                Segment::from_coords(7.0, 3.0, 7.0, -3.0),
                Intersecting(Vector::new(7.0, 0.0))
            ),
            (
                Segment::from_coords(7000.0, 3.0, 7000.0, -3.0),
                Intersecting(Vector::new(7000.0, 0.0))
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