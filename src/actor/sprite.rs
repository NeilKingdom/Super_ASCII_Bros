pub mod tile;

use crate::ascii_bros::*;
use crate::game::Game;
use crate::actor::sprite::tile::{Tile, TILE_AREA};

use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Default, Debug)]
pub struct Sprite {
    pub r#type: EntityType,     // Used to bind the sprite to an actor
    file_handle: PathBuf,       // The path at which to find the sprite sheet
    pub width: usize,           // Width of the sprite
    pub height: usize,          // Height of the sprite
    pub tile_ids: Vec<Ident>,   // A vector of Tile IDs ordered from left to right, top to bottom
    pub z_order: u8             // The z-layer priority of the sprite when rendered
}

impl Sprite {
    pub fn new(game: &mut Game, file_handle: PathBuf, z_order: u8) -> Self {
        let mut sprite: Sprite = Sprite::default();
        let sprite_as_lines = Self::load_raster_from_file(&file_handle)
            .expect("Failed to load sprite data from file");

        sprite.width = sprite_as_lines[0].len();
        sprite.height = sprite_as_lines.len();

        sprite.tile_ids = Sprite::as_tile_ids(game, &sprite_as_lines, sprite.width, sprite.height);
        sprite.file_handle = file_handle;
        sprite.z_order = z_order;

        if let Result::Err(message) = sprite.validate() {
            panic!("{}", message);
        }

        sprite
    }

    fn as_tile_ids(
        game: &mut Game, 
        sprite_as_lines: &Vec<String>, 
        sprite_width: usize, 
        sprite_height: usize
    ) -> Vec<Ident> {
        let sprite_size = sprite_width * sprite_height;

        let mut sprite_tiles: Vec<Tile> = vec![Tile::default(); sprite_size / TILE_AREA];
        let mut tile_ids: Vec<Ident> = Vec::new();

        let (mut prev_pixel_idx, mut pixel_idx, mut tile_idx): (usize, usize, usize);
        let stride = (TILE_AREA as f64).sqrt() as usize;

        for y in 0..sprite_height {
            prev_pixel_idx = (y * stride) % TILE_AREA;
            pixel_idx = prev_pixel_idx;

            for x in 0..sprite_width {
                tile_idx = ((y / stride) * (sprite_height / stride)) + (x / stride);
                sprite_tiles[tile_idx].pix_buf[pixel_idx] = sprite_as_lines[y].as_bytes()[x];

                if pixel_idx == (TILE_AREA - 1) {
                    // Only insert if pix_buf is unique
                    if !game.tile_atlas_contains(&sprite_tiles[tile_idx].pix_buf) {
                        let tile = Tile::new(
                            game.tile_atlas.len() as Ident,
                            // TODO: Implement
                            [ Color(32); TILE_AREA ],
                            sprite_tiles[tile_idx].pix_buf
                        );

                        game.tile_atlas.insert(tile.id, tile);
                        tile_ids.push(tile.id);
                    } else { // If tile is not unique, find existing key
                        tile_ids.push(
                            *game.tile_atlas.iter().find_map(|(k, v)| {
                                if v.pix_buf == sprite_tiles[tile_idx].pix_buf { 
                                    Some(k) 
                                } else { 
                                    None 
                                }
                            }).unwrap()
                        );
                    }
                }

                pixel_idx += 1;
                if pixel_idx > (((y % stride) + 1) * stride) - 1 {
                    pixel_idx = prev_pixel_idx;
                }
            }
        }

        tile_ids
    }

    fn validate(&self) -> Result<(), String> {
        let stride = (TILE_AREA as f64).sqrt() as usize;
        if self.width % stride != 0 {
            Result::Err(format!("Sprite's width ({}) is not a multiple of {}", self.width, stride))
        } else if self.height % stride != 0 {
            Result::Err(format!("Sprite's height ({}) is not a multiple of {}", self.height, stride))
        } else {
            Result::Ok(())
        }
    }

    fn load_raster_from_file(file_handle: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
        let mut sprite_as_lines = Vec::new();
        if let Ok(file) = File::open(file_handle.as_path()) {
            let reader = io::BufReader::new(file);

            // TODO: Error handling in closure
            for line in reader.lines().map(
                |line| String::from(line.unwrap().replace("\n", ""))
            ) {
                sprite_as_lines.push(line); 
            }
        }

        Ok(sprite_as_lines)
     }
}

