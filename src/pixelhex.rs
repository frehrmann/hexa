use serde::{Serialize, Deserialize};

use super::axial::Axial;
use super::Hexagons;
use super::hex::Props;


#[derive(Debug, Deserialize, Serialize)]
pub struct PixelHex {
    props: Props,
    vert_extends: (f32, f32),
    horiz_extends: Vec<(f32, f32)>,
}

impl PixelHex {
    pub fn flat(horiz_extends: &[(f32, (f32, f32))]) -> Self {

        let yext = horiz_extends.iter().map(|(y, _xx)| y)
            .fold((0f32, 0f32), |acc, &y| {
                let ymin = if y < acc.0 {
                    y
                } else {
                    acc.0
                };
                let ymax = if y > acc.1 {
                    y
                } else {
                    acc.1
                };
                (ymin, ymax)
            });

        let minx = horiz_extends.iter()
            .map(|(_y, (x1, _x2))| *x1)
            .min_by(|&v1, &v2| v1.total_cmp(&v2));

        if let Some(minx) = minx {
            let hs = horiz_extends.last().map(|(_y, (_x1, x2))| *x2).unwrap() - minx + 1.0f32;
            let vs = yext.1 - yext.0 + 1.0f32;
            PixelHex{
                props: Props::flat(hs, vs),
                horiz_extends: horiz_extends.iter().map(|(_y, x_ext)| *x_ext).collect(),
                vert_extends: yext
            }
        } else {
            PixelHex {
                props: Props::flat(0f32, 0f32),
                horiz_extends: Vec::new(),
                vert_extends: (0f32, 0f32),
            }
        }
    }
}

impl Hexagons for PixelHex {
    fn horizontal_spacing(&self) -> f32 {
        self.props.horizontal_spacing()
    }

    fn vertical_spacing(&self) -> f32 {
        self.props.vertical_spacing()
    }

    fn xy_ref(&self, qr: &Axial) -> (f32, f32) {
        self.props.xy_ref(qr)
    }

    fn xy_relative(&self, (x, y): (f32, f32)) -> (f32, f32) {
        let (x_ref, y_ref) = self.xy_ref(&self.axial((x, y)));
        (x - x_ref, y - y_ref)
    }

    fn axial(&self, (x, y): (f32, f32)) -> Axial {
        let qr = self.props.axial((x, y));
        let (_xr ,yr) = self.xy_ref(&qr);
        let dy =  y - yr;
        let dr = -1 * (dy < self.vert_extends.0) as i32 + 1 * (dy > self.vert_extends.1) as i32;
        let qr2 = qr + Axial::new(0, dr);
        let (xr ,yr) = self.xy_ref(&qr2);
        let (dx, dy) = (x - xr, y- yr);
        let ext = self.horiz_extends[(dy-self.vert_extends.0) as usize];
        let dqr2 =
            (dx < ext.0 && dy <= 0.0f32) as i32 * Axial::new(-1,  0) +
            (dx < ext.0 && dy >  0.0f32) as i32 * Axial::new(-1,  1) +
            (dx > ext.1 && dy <= 0.0f32) as i32 * Axial::new( 1, -1) +
            (dx > ext.1 && dy >  0.0f32) as i32 * Axial::new( 1,  0);
        qr2 + dqr2
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_null() {
        let v = Vec::new();
        let h = PixelHex::flat(&v);
        assert_eq!(h.horizontal_spacing(), 0f32);
        assert_eq!(h.vertical_spacing(), 0f32);
    }

    #[test]
    fn test() {
        let v = vec![
            (-1.0f32, ( 0f32, 1f32)),
            ( 0.0f32, (-1f32, 2f32)),
            ( 1.0f32, (-1f32, 2f32)),
            ( 2.0f32, ( 0f32, 1f32))
        ];
        let h = PixelHex::flat(&v);
        assert_eq!(h.horizontal_spacing(), 3f32);
        assert_eq!(h.vertical_spacing(), 4f32);

        assert_eq!(h.xy_ref(&Axial::new( 0,  0)), ( 0f32,  0f32));
        assert_eq!(h.xy_ref(&Axial::new(-1,  0)), (-3f32, -2f32));
        assert_eq!(h.xy_ref(&Axial::new( 0, -1)), ( 0f32, -4f32));
        assert_eq!(h.xy_ref(&Axial::new( 1, -1)), ( 3f32, -2f32));
        assert_eq!(h.xy_ref(&Axial::new( 1,  0)), ( 3f32,  2f32));
        assert_eq!(h.xy_ref(&Axial::new( 0,  1)), ( 0f32,  4f32));
        assert_eq!(h.xy_ref(&Axial::new(-1,  1)), (-3f32,  2f32));

        assert_eq!(h.axial(( 0f32,  0f32)), Axial::new(0, 0));

        assert_eq!(h.axial(( 0f32, -1f32)), Axial::new(0, 0));
        assert_eq!(h.axial(( 1f32, -1f32)), Axial::new(0, 0));

        assert_eq!(h.axial(( 2f32,  0f32)), Axial::new(0, 0));
        assert_eq!(h.axial(( 2f32,  1f32)), Axial::new(0, 0));

        assert_eq!(h.axial(( 1f32,  2f32)), Axial::new(0, 0));
        assert_eq!(h.axial(( 0f32,  2f32)), Axial::new(0, 0));

        assert_eq!(h.axial((-1f32,  1f32)), Axial::new(0, 0));
        assert_eq!(h.axial((-1f32,  0f32)), Axial::new(0, 0));

        assert_eq!(h.axial((-2f32,  0f32)), Axial::new(-1,  0));
        assert_eq!(h.axial((-1f32, -1f32)), Axial::new(-1,  0));

        assert_eq!(h.axial(( 0f32, -2f32)), Axial::new( 0, -1));
        assert_eq!(h.axial(( 1f32, -2f32)), Axial::new( 0, -1));

        assert_eq!(h.axial(( 2f32, -1f32)), Axial::new( 1, -1));
        assert_eq!(h.axial(( 3f32,  0f32)), Axial::new( 1, -1));

        assert_eq!(h.axial(( 3f32,  1f32)), Axial::new( 1,  0));
        assert_eq!(h.axial(( 2f32,  2f32)), Axial::new( 1,  0));

        assert_eq!(h.axial(( 1f32,  3f32)), Axial::new( 0,  1));
        assert_eq!(h.axial(( 0f32,  3f32)), Axial::new( 0,  1));

        assert_eq!(h.axial((-1f32,  2f32)), Axial::new(-1,  1));
        assert_eq!(h.axial((-2f32,  1f32)), Axial::new(-1,  1));
    }

    static PIXELHEXRON: &str = "(
        props: (
            top: FLAT,
            vert_spacing: 32.0,
            horz_spacing: 29.0,
        ),
        vert_extends: (-16.0, 15.0),
        horiz_extends: [
            (-11.0, 10.0),
            (-11.0, 10.0),
            (-12.0, 11.0),
            (-12.0, 11.0),
            (-13.0, 12.0),
            (-13.0, 12.0),
            (-14.0, 13.0),
            (-14.0, 13.0),
            (-15.0, 14.0),
            (-15.0, 14.0),
            (-16.0, 15.0),
            (-16.0, 15.0),
            (-17.0, 16.0),
            (-17.0, 16.0),
            (-18.0, 17.0),
            (-18.0, 17.0),
            (-18.0, 17.0),
            (-18.0, 17.0),
            (-17.0, 16.0),
            (-17.0, 16.0),
            (-16.0, 15.0),
            (-16.0, 15.0),
            (-15.0, 14.0),
            (-15.0, 14.0),
            (-14.0, 13.0),
            (-14.0, 13.0),
            (-13.0, 12.0),
            (-13.0, 12.0),
            (-12.0, 11.0),
            (-12.0, 11.0),
            (-11.0, 10.0),
            (-11.0, 10.0),
        ],
    )";

    #[test]
    fn test_bigger() {
        let h: PixelHex = ron::from_str(PIXELHEXRON).unwrap();
        assert_eq!(h.horizontal_spacing(), 29f32);
        assert_eq!(h.vertical_spacing(), 32f32);
        let ymin = h.vert_extends.0;
        for x in -20..=20 {
            assert_ne!(h.axial((x as f32, ymin - 1f32)), Axial::new(0, 0), "h.axial(({},{}))", x as f32, ymin - 1.0f32);
        }
        for (i, (x1, x2)) in h.horiz_extends.iter().enumerate() {
            for x in (*x1 as i32-3)..=(*x1 as i32 - 1) {
                assert_ne!(h.axial((x as f32, ymin + i as f32)), Axial::new(0,0 ), "h.axial(({},{}))", x as f32, ymin);
            }
            for x in (*x1 as i32)..=(*x2 as i32) {
                assert_eq!(h.axial((x as f32, ymin + i as f32)), Axial::new(0,0 ), "h.axial(({},{}))", x as f32, ymin);
            }
            for x in (*x2 as i32+1)..=(*x2 as i32 +5) {
                assert_ne!(h.axial((x as f32, ymin + i as f32)), Axial::new(0,0 ), "h.axial(({},{}))", x as f32, ymin);
            }
        }
        for x in -20..=20 {
            assert_ne!(h.axial((x as f32, h.vert_extends.1 + 1f32)), Axial::new(0, 0), "h.axial(({},{}))", x as f32, ymin);
        }
    }

}
