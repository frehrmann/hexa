pub mod axial;

/// Axial coordinates for hexagon maps.
#[derive(Default, Debug, Clone, Copy)]
pub struct HexSizes {
    width: f32,
    height: f32,
    vert_spacing: f32,
    horz_spacing: f32,
}
