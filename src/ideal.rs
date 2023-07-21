/// An ideal hexagon
/// Following the great page https://www.redblobgames.com/grids/hexagons/
use super::axial::Axial;
use super::HexTop;


/// Properties of a hexagon with a given size.
#[derive(Copy, Clone, Debug)]
pub struct HexProps {
    top: HexTop,
    size: f32,
    width: f32,
    height: f32,
    inner_radius: f32,
    outer_radius: f32,
    vert_spacing: f32,
    horz_spacing: f32,
    points: [(f32, f32);6],
}

const SQRT3 : f32 = 1.732050807568877193176604123436845839023590087890625f32;
const SQRT3BY2 : f32 = SQRT3/2f32;
const SQRT3BY3 : f32 = SQRT3/3f32;
const ONETHIRD : f32 = 1f32/3f32;
const TWOTHIRD : f32 = 2f32/3f32;

impl HexProps {

    /// Creates `HexProps` for a flat top hexagon with `size`.
    /// The size is the radius of the outer circle which goes through the edges of the hexagon.
    pub fn flat(size: f32) -> Self {
        let mut pts = [(0f32, 0f32); 6];
        for i in 0..=5 {
            let ang = (i as f32 * 60f32).to_radians();
            pts[i] = (size*ang.cos(), size*ang.sin());
        }
        HexProps {
            top: HexTop::FLAT,
            size: size,
            width: 2f32 * size,
            height: SQRT3 * size,
            inner_radius: SQRT3/2f32 * size,
            outer_radius: size,
            vert_spacing: SQRT3 * size,
            horz_spacing: 3f32/2f32 * size,
            points: pts,
        }
    }

    /// Creates `HexProps` for a pointy top hexagon with `size`.
    /// The size is the radius of the outer circle which goes through the edges of the hexagon.
    pub fn pointy(size: f32) -> Self {
        let mut pts = [(0f32, 0f32); 6];
        for i in 0..=5 {
            let ang = (i as f32 * 60f32 + 30f32).to_radians();
            pts[i] = (size*ang.cos(), size*ang.sin());
        }
        HexProps {
            top: HexTop::POINTY,
            size: size,
            width: SQRT3 * size,
            height: 2f32 * size,
            inner_radius: SQRT3/2f32 * size,
            outer_radius: size,
            vert_spacing: 3f32/2f32 * size,
            horz_spacing: SQRT3 * size,
            points: pts,
        }
    }

    pub fn size(&self) -> f32 { self.size }

    pub fn points(&self) -> &[(f32, f32)] { &self.points }

    /// Outer radius of the hexagon
    pub fn outer(&self) -> f32 { self.outer_radius }

    /// Convert axial (q,r) coordinates into (x,y) coordinates for the hexagons center.
    pub fn axial2xy(&self, qr : Axial) -> (f32, f32) {
        let qr = qr.to_f32s();
        let mat = match self.top {
            HexTop::FLAT => [[1.5f32, SQRT3BY2], [0f32, SQRT3]],
            HexTop::POINTY => [[SQRT3, 0f32], [SQRT3BY2, 1.5f32]],
        };
        let x = self.size * (mat[0][0]*qr.0+mat[0][1]*qr.1);
        let y = self.size * (mat[1][0]*qr.0+mat[1][1]*qr.1);
        (x, y)
    }

    // Convert pixel coordinates to axial coordinates
    pub fn xy2axial(&self, xy : impl Into<(f32, f32)>) -> Axial {
        let xy = xy.into();
        let mat = match self.top {
            HexTop::FLAT => [[TWOTHIRD, -ONETHIRD], [0f32, SQRT3BY3]],
            HexTop::POINTY => [[SQRT3BY3, 0f32], [-ONETHIRD, TWOTHIRD]],
        };
        let qf = (mat[0][0]*xy.0+mat[0][1]*xy.1) / self.size;
        let rf = (mat[1][0]*xy.0+mat[1][1]*xy.1) / self.size;
        Axial::from((qf,rf))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_props() {
        assert_eq!(HexProps::flat(13.0f32).width, 26.0f32);
    }
}