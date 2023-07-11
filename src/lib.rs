use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul};

/// Axial coordinates for hexagon maps.
#[derive(Default, Debug, Clone, Copy)]
pub struct HexSizes {
    width: f32,
    height: f32,
    vert_spacing: f32,
    horz_spacing: f32,
}

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

        let one = q_diff > r_diff && q_diff > s_diff;
        let two = r_diff > s_diff;

        let q = q as i32 * (!one as i32) - (r as i32 + s as i32) * (one as i32);
        let r = r as i32 * (!two as i32) - (q as i32 + s as i32) * (two as i32);

        Axial::new(q, r)
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
}
