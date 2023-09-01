pub mod axial;
//pub mod ideal;
pub mod hex;
pub mod pixelhex;

use axial::Axial;
use serde::{Deserialize, Serialize};

/// Defines how the hexagon is orientated.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum HexTop {
    FLAT,
    POINTY,
}

pub trait Hexagons {

    /// Return the spacing in x direction.
    fn horizontal_spacing(&self) -> f32;

    /// Return the spacing in y direction.
    fn vertical_spacing(&self) -> f32;

    /// Compute the x,y reference of a hexagon from Axial coordinates.
    fn xy_ref(&self, qr: &Axial) -> (f32, f32);

    /// Compute the xy value relative to the center of the next hexagon.
    fn xy_relative(&self, xy: (f32, f32)) -> (f32, f32);

    /// Compute the axial index of a hexagon from a x,y coordinate.
    fn axial(&self, xy: (f32, f32)) -> Axial;

}
