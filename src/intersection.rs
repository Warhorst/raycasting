use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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
        self.x * rhs.y - self.y * rhs.x
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

#[derive(Copy, Clone, Debug)]
struct Segment {
    a: Vec2,
    b: Vec2,
}

impl Segment {
    fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Segment {
            a: Vec2 {
                x: x0,
                y: y0,
            },
            b: Vec2 {
                x: x1,
                y: y1,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum Intersect {
    Intersecting(Vec2),
    Collinear,
    NotIntersecting,
}

/// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
fn get_segment_intersection(
    s0: Segment,
    s1: Segment,
) -> Intersect {
    let p = s0.a;
    let q = s1.a;
    let r = s0.b - s0.a;
    let s = s1.b - s1.a;
    let r_cross_s = r * s;
    let q_minus_p = q - p;
    let q_minus_p_cross_r = q_minus_p * r;

    let t = q_minus_p * (s / r_cross_s);
    let u = q_minus_p * (r / r_cross_s);

    println!("s0: {:?}", s0);
    println!("s1: {:?}", s1);
    println!("p: {:?}", p);
    println!("q: {:?}", q);
    println!("r: {:?}", r);
    println!("s: {:?}", s);
    println!("t: {t}");
    println!("u: {u}");
    println!("r * s: {}", r * s);
    println!("(q - p) * r: {}", q_minus_p_cross_r);

    if r_cross_s == 0.0 && q_minus_p_cross_r == 0.0 {
        // case 1: collinear
        return Intersect::Collinear;
    }

    if r_cross_s == 0.0 && q_minus_p_cross_r != 0.0 {
        // case 2: lines are parallel and not intersecting
        return Intersect::NotIntersecting;
    }

    if r_cross_s != 0.0 && (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        // case 3: intersection
        return Intersect::Intersecting(p + r * t);
    }

    Intersect::NotIntersecting
}

#[cfg(test)]
mod tests {
    use crate::intersection::{get_segment_intersection, Segment, Vec2};
    use crate::intersection::Intersect::*;

    #[test]
    fn intersection_works() {
        let segment = Segment::new(0.0, 0.0, 5.0, 0.0);

        [
            (
                Segment::new(3.0, 3.0, 3.0, -3.0),
                Intersecting(Vec2::new(3.0, 0.0))
            ),
            (
                Segment::new(0.0, 3.0, 0.0, -3.0),
                Intersecting(Vec2::new(0.0, 0.0))
            ),
            (
                Segment::new(5.0, 3.0, 5.0, -3.0),
                Intersecting(Vec2::new(5.0, 0.0))
            ),
            (
                Segment::new(0.0, 0.0, 3.0, 0.0),
                Collinear
            )
        ]
            .into_iter()
            .for_each(|(s, intersection)| assert_eq!(get_segment_intersection(segment, s), intersection))
    }
}