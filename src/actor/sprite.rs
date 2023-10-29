pub mod tile;

use crate::ascii_bros::*;
use crate::game::Game;
use crate::actor::sprite::tile::{Tile, TILE_AREA};

use std::error::Error;
use std::path::PathBuf;
use std::fs;

trait StringExtensions {
    fn as_tile_ids(&self, game: &mut Game) -> Vec<Ident>;
}

// TODO: Add tests for this function
impl StringExtensions for String {
    fn as_tile_ids(&self, game: &mut Game) -> Vec<Ident> {
        let lines: Vec<&str> = self.split("\n").collect();
        let len = self.replace("\n", "").len();

        let mut pix_bufs: Vec<Tile> = vec![Tile::default(); len / TILE_AREA];
        let mut tile_ids: Vec<Ident> = Vec::new();

        let (mut prev_pixel_idx, mut pixel_idx, mut tile_idx): (usize, usize, usize);
        let stride = (TILE_AREA as f64).sqrt() as usize;

        for y in 0..(lines.len() - 1) {
            prev_pixel_idx = (y * stride) % TILE_AREA;
            pixel_idx = prev_pixel_idx;

            for x in 0..(lines.len() - 1) {
                tile_idx = ((y / stride) * (lines.len() / stride)) + (x / stride);
                pix_bufs[tile_idx].pix_buf[pixel_idx] = lines[y].as_bytes()[x];

                if pixel_idx == (TILE_AREA - 1) {
                    let tile = Tile::new(
                        { let tmp = game.next_tile_id; game.next_tile_id += 1; tmp },
                        stride,
                        // TODO: Implement
                        [ Color(32); TILE_AREA ],
                        pix_bufs[tile_idx].pix_buf
                    );

                    game.tile_atlas.insert(tile.id, tile);
                    tile_ids.push(tile.id);
                }

                pixel_idx += 1;
                if pixel_idx > (((y % stride) + 1) * stride) - 1 {
                    pixel_idx = prev_pixel_idx;
                }
            }
            println!();
        }

        tile_ids
    }
}

#[derive(Default, Debug)]
pub struct Sprite {
    id: Ident,                  // The sprite's unique identifier
    file_handle: PathBuf,       // The path at which to find the sprite data
    tile_data: Vec<Ident>,      // A vector of Tile IDs ordered from left to right, top to bottom
    x_pos: f32,                 // The current x position of the sprite (in sub-pixels)
    y_pos: f32,                 // The current y position of the sprite (in sub-pixels)
    z_order: u8                 // The z layer priority of the sprite when rendered
}

impl Sprite {
    fn default(game: &mut Game) -> Self {
        Sprite {
            id: { let tmp = game.next_sprite_id; game.next_sprite_id += 1; tmp },
            file_handle: PathBuf::new(),
            tile_data: Vec::new(),
            x_pos: 0.0,
            y_pos: 0.0,
            z_order: 0
        }
    }

    fn validate(&self) -> Result<(), String> {
        // Sprite tile data size should be a perfect square and multiple of TILE_AREA
        if self.tile_data.len() == 0 ||
           (self.tile_data.len() as f64).sqrt() % (TILE_AREA as f64) != 0.0 {
            Result::Err(format!("Sprite {}'s tile data is of an invalid size", self.id))
        } else {
            Result::Ok(())
        }
    }

    fn load_raster_from_file(file_handle: &PathBuf) -> Result<String, Box<dyn Error>> {
        let pix_buf = fs::read_to_string(file_handle.as_path())?.replace("@", " ");
        println!("{}", pix_buf);
        Ok(pix_buf)
     }

    pub fn new(game: &mut Game, file_handle: PathBuf, z_order: u8) -> Self {
        let pix_buf = Self::load_raster_from_file(&file_handle)
            .expect("Failed to load sprite data from file");

        let mut sprite: Sprite = Sprite::default(game);
        sprite.file_handle = file_handle;
        sprite.tile_data = pix_buf.as_tile_ids(game);
        sprite.z_order = z_order;

        let result = sprite.validate();
        match result {
            Result::Err(message) => { panic!("{}", message); },
            _ => ()
        }

        sprite
    }
}

