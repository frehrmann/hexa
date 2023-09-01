use serde::{Deserialize, Serialize};

/// Pixel hexagons might have a bit different spacings.

use super::axial::Axial;
use super::{HexTop, Hexagons};

#[derive(Debug, Deserialize, Serialize)]
pub struct Props {
    top: HexTop,
    vert_spacing: f32,
    horz_spacing: f32,
}

impl Props {
    pub fn new(top: HexTop, vert_spacing: f32, horz_spacing: f32) -> Self {
        Props {
            top,
            vert_spacing,
            horz_spacing
        }
    }

    pub fn flat(hs: f32, vs: f32) -> Self {
        Props::new(HexTop::FLAT, vs, hs)
    }

    pub fn pointy(hs: f32, vs: f32) -> Self {
        Props::new(HexTop::POINTY, vs, hs)
    }

    fn xy_flat(&self, qr: &Axial) -> (f32, f32) {
        let (qf, rf) = qr.to_f32s();
        let x = qf * self.horz_spacing;
        let y = (0.5f32 * qf + rf) * self.vert_spacing;
        (x, y)
    }

    fn xy_pointy(&self, qr: &Axial) -> (f32, f32) {
        let (qf, rf) = qr.to_f32s();
        let x = (qf+rf/2f32) * self.horz_spacing;
        let y = rf * self.vert_spacing;
        (x, y)
    }

    fn pointy_qr_from_xy(&self, (x, y): (f32, f32)) -> Axial {
        let q =  self.horz_spacing.recip() * x - 0.5f32 * self.vert_spacing.recip() * y;
        let r = self.vert_spacing.recip() * y;
        Axial::from((q, r))
    }

    fn flat_qr_from_xy(&self, (x, y): (f32, f32)) -> Axial {
        let q = self.horz_spacing.recip() * x;
        let r = - 0.5f32 * self.horz_spacing.recip() * x + self.vert_spacing.recip() * y;
        Axial::from((q, r))
    }

}

impl Hexagons for Props {
    fn horizontal_spacing(&self) -> f32 {
        self.horz_spacing
    }

    fn vertical_spacing(&self) -> f32 {
        self.vert_spacing
    }

    fn xy_ref(&self, qr: &Axial) -> (f32, f32) {
        match self.top {
            HexTop::FLAT => self.xy_flat(qr),
            HexTop::POINTY => self.xy_pointy(qr),
        }
    }

    fn xy_relative(&self, xy: (f32, f32)) -> (f32, f32) {
        let (xc, yc) = self.xy_ref(&self.axial(xy));
        (xy.0 -xc, xy.1 -yc)
    }

    fn axial(&self, xy: (f32, f32)) -> Axial {
        match self.top {
            HexTop::FLAT => self.flat_qr_from_xy(xy),
            HexTop::POINTY => self.pointy_qr_from_xy(xy),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xy() {
        let h = Props::flat(7f32, 10f32);
        assert_eq!(h.xy_ref(&Axial::default()), (0f32, 0f32));
        assert_eq!(h.xy_ref(&Axial::new(1, 1)), (7f32, 15f32));
        assert_eq!(h.xy_ref(&Axial::new(-2, 1)), (-14f32, 0f32));

    }

    #[test]
    fn test_qr() {
        let h = Props::flat(7f32, 10f32);
        assert_eq!(h.axial((2f32, 2f32)), Axial::new(0, 0));
        assert_eq!(h.axial((7f32, 15f32)), Axial::new(1, 1));
        assert_eq!(h.axial((-13f32, 1f32)), Axial::new(-2, 1));
    }
}