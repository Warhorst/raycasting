use bevy::prelude::*;

use crate::raycasting::IntersectionStatus::*;

#[derive(Copy, Clone, PartialEq)]
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
            let t0 = q_minus_p.dot(r) / (r.dot(r));
            let t1 = t0 + ((s.dot(r)) / (r.dot(r)));

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
        direction: Vec2,
    ) -> Self {
        Ray {
            origin,
            direction,
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
            let t0 = q_minus_p.dot(r) / (r.dot(r));
            let t1 = t0 + ((s.dot(r)) / (r.dot(r)));

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
            self.direction.x * radians.cos() - self.direction.y * radians.sin(),
            self.direction.x * radians.sin() + self.direction.y * radians.cos(),
        );

        Ray {
            origin: self.origin,
            direction: rotated_direction,
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
    origin: Vec2,
    segments: Vec<Segment>,
) -> Vec<Triangle> {
    let intersection_points = calculate_intersection_points(origin, segments);

    let mut triangles = intersection_points
        .windows(2)
        .map(|nodes| Triangle {
            a: (origin.x, origin.y),
            b: (nodes[0].x, nodes[0].y),
            c: (nodes[1].x, nodes[1].y),
        })
        .collect::<Vec<_>>();

    let first = intersection_points.first().unwrap();
    let last = intersection_points.last().unwrap();

    triangles.push(Triangle {
        a: (origin.x, origin.y),
        b: (first.x, first.y),
        c: (last.x, last.y),
    });

    triangles
}

/// Return every intersection point of rays from origin to every point of the segments and the segments itself.
/// The intersection points are ordered by angle.
///
/// TODO: please kill me (or better: refactor)
/// TODO: jittery. Maybe a floating point issue?
pub fn calculate_intersection_points(
    origin: Vec2,
    segments: Vec<Segment>,
) -> Vec<Vec2> {
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

    let mut intersections = Vec::with_capacity(points.len());
    let mut extra_rays = Vec::with_capacity(points.len() * 2);

    for point in points {
        let direction = point - origin;
        let origin_to_point = Ray::new(origin, direction);

        let mut nearest_intersection = None;
        let mut nearest_distance = f32::MAX;
        let mut hit_segment = None;

        for segment in &segments {
            // TODO Collinear intersecting is a special case
            if let Intersecting(intersection) = origin_to_point.calculate_intersection(*segment) {
                let distance_to_intersection = calculate_distance(origin, intersection);

                if distance_to_intersection < nearest_distance {
                    nearest_intersection = Some(intersection);
                    nearest_distance = distance_to_intersection;
                    hit_segment = Some(*segment)
                }
            }
        }

        if let Some(intersection) = nearest_intersection {
            intersections.push(intersection);

            if intersection == point {
                extra_rays.push((hit_segment.unwrap(), origin_to_point.rotate(-0.01)));
                extra_rays.push((hit_segment.unwrap(), origin_to_point.rotate(0.01)));
            }
        }
    }

    for (original_segment, ray) in extra_rays {
        let mut nearest_intersection = None;
        let mut nearest_distance = f32::MAX;
        let mut hit_segment = None;

        for segment in &segments {
            if let Intersecting(intersection) = ray.calculate_intersection(*segment) {
                let distance_to_intersection = calculate_distance(origin, intersection);

                if distance_to_intersection < nearest_distance {
                    nearest_intersection = Some(intersection);
                    nearest_distance = distance_to_intersection;
                    hit_segment = Some(*segment)
                }
            }
        }

        if let Some(intersection) = nearest_intersection {
            if hit_segment.unwrap() != original_segment {
                intersections.push(intersection)
            }
        }
    }

    intersections.sort_by(|p1, p2| {
        let angle_0 = calculate_angle(origin, *p1);
        let angle_1 = calculate_angle(origin, *p2);
        angle_0.total_cmp(&angle_1)
    });

    intersections
}

fn calculate_distance(
    p1: Vec2,
    p2: Vec2,
) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

fn calculate_angle(
    p1: Vec2,
    p2: Vec2,
) -> f32 {
    let angle_rad = (p2.y - p1.y).atan2(p2.x - p1.x);
    angle_rad.to_degrees()
}

/// Enables Vec2 to implement cross product.
trait CrossProduct {
    fn cross_product(&self, other: Self) -> f32;
}

impl CrossProduct for Vec2 {
    fn cross_product(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::raycasting::IntersectionStatus::*;
    use crate::raycasting::{Ray, Segment};

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