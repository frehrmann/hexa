/// Following the great page https://www.redblobgames.com/grids/hexagons/

use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Axial {
    q: i32,
    r: i32
}

impl Axial {
    pub fn new(q: impl Into<i32>, r: impl Into<i32>) -> Self {
        Axial { q: q.into(), r: r.into()}
    }

    /// Compute the third of the hexagonal cube coordinates.
    pub fn s(&self) -> i32 {
        -self.q-self.r
    }

    pub fn length(&self) -> u32 {
        (self.q.abs() + (self.q+self.r).abs() + self.r.abs()) as u32 / 2
    }

    pub fn distance_to(&self, other: Self) -> u32 {
        (*self - other).length()
    }

    pub fn lerp(&self, other: Self, t: f32) -> (f32, f32) {
        let (q1, r1) = *self * (1f32 - t);
        let (q2, r2) = other * t;
        (q1+q2, r1+r2)
    }

    pub fn point_on_line(p1: Self, p2: Self, dist: f32) -> Self {
        let dist_p1_p2 = p1.distance_to(p2) as f32;
        Axial::from(p1.lerp(p2, dist/dist_p1_p2))
    }

    pub fn circle(&self, hex_radius: u32) -> CircleAroundHex {
        CircleAroundHex::new(*self, hex_radius)
    }

    pub fn neighbours(&self) -> CircleAroundHex {
        self.circle(1)
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.q, self.r)
    }

    pub fn to_f32s(&self) -> (f32, f32) {
        (self.q as f32, self.r as f32)
    }

}

impl AddAssign for Axial {
    fn add_assign(&mut self, rhs: Self) {
        self.q += rhs.q;
        self.r += rhs.r;
    }
}

impl Add for Axial {
    type Output = Axial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl SubAssign for Axial {

    fn sub_assign(&mut self, rhs: Self) {
        self.q  -= rhs.q;
        self.r -= rhs.r;
    }
}

impl Sub for Axial {
    type Output = Axial;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl MulAssign<i32> for Axial {

    fn mul_assign(&mut self, rhs: i32) {
        self.q *= rhs;
        self.r *= rhs;
    }

}

impl Mul<i32> for Axial {
    type Output = Axial;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl Mul<f32> for Axial {
    type Output = (f32, f32);

    fn mul(self, rhs: f32) -> Self::Output {
        (self.q as f32 * rhs, self.r as f32 * rhs)
    }
}

impl Mul<Axial> for i32 {
    type Output = Axial;

    fn mul(self, rhs: Axial) -> Self::Output {
       rhs * self
    }
}

impl Mul<Axial> for f32 {
    type Output = (f32, f32);

    fn mul(self, rhs: Axial) -> Self::Output {
        rhs * self
    }
}

impl From<(i32, i32)> for Axial {
    fn from((q,r): (i32, i32)) -> Self {
        Axial::new(q, r)
    }
}

impl From<(f32, f32)> for Axial {
    fn from ((q_f, r_f): (f32, f32)) -> Self {
        let s_f = -q_f - r_f;

        let q = q_f.round();
        let r = r_f.round();
        let s = s_f.round();

        let q_diff = (q - q_f).abs();
        let r_diff = (r - r_f).abs();
        let s_diff = (s - s_f).abs();

        let calc_q_b = q_diff > r_diff && q_diff > s_diff;
        let calc_r = (!calc_q_b && r_diff > s_diff) as i32;
        let calc_q = calc_q_b as i32;

        let q_out = q as i32 * (1-calc_q) - (r as i32 + s as i32) * calc_q;
        let r_out = r as i32 * (1-calc_r) - (q as i32 + s as i32) * calc_r;

        Axial::new(q_out, r_out)
    }
}


/// Circle Iterator
struct HexCircle {
    hex_radius: i32,
    leg_idx: usize,
    hex_idx: i32,
}

impl HexCircle {
    fn new(hex_radius: u32) -> Self {
        HexCircle {
            hex_radius: hex_radius as i32,
            leg_idx: 0usize,
            hex_idx: 0,
        }
    }

    fn qr(&self) -> Option<(i32, i32)> {
        match self.leg_idx {
            0 => Some((-self.hex_idx,                     self.hex_radius)),
            1 => Some((-self.hex_radius,                  self.hex_radius-self.hex_idx)),
            2 => Some((-self.hex_radius + self.hex_idx,  -self.hex_idx)),
            3 => Some(( self.hex_idx,                    -self.hex_radius)),
            4 => Some(( self.hex_radius,                 -self.hex_radius+self.hex_idx)),
            5 => Some(( self.hex_radius-self.hex_idx,     self.hex_idx)),
            _ => None,
        }
    }
}

impl Iterator for HexCircle {
    type Item = Axial;

    fn next(&mut self) -> Option<Self::Item> {
        if self.leg_idx > 5 {
            return None;
        }
        let qr = self.qr();
        if self.hex_radius == 0 {
            self.leg_idx = 6;
        } else if self.hex_idx < (self.hex_radius - 1) {
            self.hex_idx += 1;
        } else {
            self.hex_idx = 0;
            self.leg_idx += 1;
        }
        qr.map(|v| Axial::from(v))
    }
}

/// Circle around a given hexagon
pub struct CircleAroundHex {
    circle: HexCircle,
    center: Axial,
}

impl Iterator for CircleAroundHex {
    type Item = Axial;

    fn next(&mut self) -> Option<Self::Item> {
        self.circle.next().map(|v| v + self.center)
    }
}

impl CircleAroundHex {
    pub fn new(center: Axial, radius: u32) -> Self {
        CircleAroundHex {
            circle: HexCircle::new(radius),
            center
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s() {
        let a = Axial::new(1, 2);
        assert_eq!(a.s(), -3);
    }

    #[test]
    fn test_from() {
        let a = Axial::from((5, -4));
        assert_eq!((a.q, a.r), (5, -4));

        let a: Axial = Axial::from((5.4f32, 3.2f32));
        assert_eq!((a.q, a.r), (6, 3));

        let a: Axial = Axial::from((2.3f32, -13.6f32));
        assert_eq!((a.q, a.r), (2, -13));
    }

    #[test]
    fn test_ops() {
        let a = Axial::default();
        let b: Axial = Axial::new(1, 2);
        assert_eq!(a+b, Axial::new(1, 2));
        assert_eq!(a-b, Axial::new(-1, -2));
        assert_eq!(2*b, Axial::new(2, 4));
        assert_eq!(-0.5f32*b, (-0.5f32, -1.0f32));
    }

    #[test]
    fn test_math() {
        let a = Axial::new(-1, 3);
        assert_eq!(a.length(), 3);
        assert_eq!(Axial::default().length(), 0);
        assert_eq!(a.distance_to(Axial::default()), 3);
        let b = Axial::new(2, -2);
        assert_eq!(b.distance_to(a), 5);
        assert_eq!(a.distance_to(b), 5);
        let c = Axial::new(4, 6);
        assert_eq!(c.distance_to(a), 8);
        assert_eq!(a.lerp(c, 0.5f32), (1.5f32, 4.5f32));
        assert_eq!(Axial::point_on_line(a, c, 4f32), Axial::from((1.5f32, 4.5f32)));
    }

    #[test]
    fn test_circle() {
        let mut ai = HexCircle::new(0);
        assert_eq!(ai.next(), Some(Axial::default()));
        assert!(ai.next().is_none());
        let mut ai = HexCircle::new(2);
        assert_eq!(ai.next(), Some(Axial::new(0, 2)));
        assert_eq!(ai.next(), Some(Axial::new(-1, 2)));
        assert_eq!(ai.next(), Some(Axial::new(-2, 2)));
        assert_eq!(ai.next(), Some(Axial::new(-2, 1)));
        assert_eq!(ai.next(), Some(Axial::new(-2, 0)));
        assert_eq!(ai.next(), Some(Axial::new(-1, -1)));
        assert_eq!(ai.next(), Some(Axial::new(0, -2)));
        assert_eq!(ai.next(), Some(Axial::new(1, -2)));
        assert_eq!(ai.next(), Some(Axial::new(2, -2)));
        assert_eq!(ai.next(), Some(Axial::new(2, -1)));
        assert_eq!(ai.next(), Some(Axial::new(2, 0)));
        assert_eq!(ai.next(), Some(Axial::new(1, 1)));
        assert!(ai.next().is_none());
        let mut ai = Axial::new(1, -1).circle(2);
        assert_eq!(ai.next(), Some(Axial::new(1, 1)));
        assert_eq!(ai.next(), Some(Axial::new(0, 1)));
        assert_eq!(ai.next(), Some(Axial::new(-1, 1)));
        assert_eq!(ai.next(), Some(Axial::new(-1, 0)));
        assert_eq!(ai.next(), Some(Axial::new(-1, -1)));
        assert_eq!(ai.next(), Some(Axial::new(0, -2)));
        assert_eq!(ai.next(), Some(Axial::new(1, -3)));
        assert_eq!(ai.next(), Some(Axial::new(2, -3)));
        assert_eq!(ai.next(), Some(Axial::new(3, -3)));
        assert_eq!(ai.next(), Some(Axial::new(3, -2)));
        assert_eq!(ai.next(), Some(Axial::new(3, -1)));
        assert_eq!(ai.next(), Some(Axial::new(2, 0)));
        assert!(ai.next().is_none());
    }
}
