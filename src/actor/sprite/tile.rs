use crate::ascii_bros::*;

pub const TILE_AREA: usize = 4;

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: Ident,                      // The tile's unique identifier
    pub stride: usize,                  // Stride for both color_buf and pix_buf
    pub color_buf: [Color; TILE_AREA],  // The color data corresponding to each pixel in the tile
    pub pix_buf: [u8; TILE_AREA],       // The "pixel" data i.e. sequence of characters
}

impl Tile {
    pub fn new(
        id: Ident,
        stride: usize,
        color_buf: [Color; TILE_AREA],
        pix_buf: [u8; TILE_AREA]
    ) -> Self {
        Tile { id, stride, color_buf, pix_buf }
    }
}

