use crate::{IntersectionStatus, Segment, Vec2};
use crate::IntersectionStatus::*;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec2,
    direction: Vec2,
}

pub fn calculate_ray_segment_intersection(
    ray: Ray,
    segment: Segment,
) -> IntersectionStatus {
    let p = ray.origin;
    let q = segment.a;
    let r = ray.direction;
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

#[cfg(test)]
mod tests {
    use crate::ray_intersection::{calculate_ray_segment_intersection, Ray};
    use crate::{Segment, Vec2};
    use crate::IntersectionStatus::*;

    #[test]
    fn intersection_works() {
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
        ].into_iter().for_each(|(l, intersection)| assert_eq!(calculate_ray_segment_intersection(ray, l), intersection))
    }
}