pub mod axial;
//pub mod ideal;
pub mod hex;

/// Defines how the hexagon is orientated.
#[derive(Copy, Clone, Debug)]
pub enum HexTop {
    FLAT,
    POINTY,
}
