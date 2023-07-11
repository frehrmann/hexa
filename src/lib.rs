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
