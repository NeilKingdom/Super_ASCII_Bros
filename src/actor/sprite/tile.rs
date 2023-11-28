use crate::ascii_bros::*;

use std::iter::Iterator;
use std::ops::{Index, IndexMut};

pub const TILE_AREA: usize = 4;

#[derive(Default, Clone, Copy)]
pub struct PixBuf([u8; TILE_AREA]);

impl PixBuf {
    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.0.iter()
    }
}

impl Iterator for PixBuf {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.iter().next().is_some() {
            // If there are more elements, return the next one.
            Some(self.0[0])
        } else {
            // If there are no more elements, signal the end of iteration.
            None
        }
    }
}

// Implement PartialEq trait to overload == operator for [u8; TILE_AREA];
impl std::cmp::PartialEq for PixBuf {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

// Implement the Index trait to allow indexing into PixBuf
impl Index<usize> for PixBuf {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Implement the IndexMut trait to allow mutable indexing into PixBuf
impl IndexMut<usize> for PixBuf {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: Ident,                      // The tile's unique identifier
    pub color_buf: [Color; TILE_AREA],  // The color data corresponding to each pixel in the tile
    pub pix_buf: PixBuf,                // The "pixel" data i.e. sequence of characters
}

impl Tile {
    pub fn new(
        id: Ident,
        color_buf: [Color; TILE_AREA],
        pix_buf: PixBuf,
    ) -> Self {
        Tile { id, color_buf, pix_buf }
    }
}

