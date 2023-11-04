use crate::Window;
use crate::ascii_bros::*;
use crate::actor::sprite::tile::{Tile, PixBuf};

use std::collections::HashMap;
use itertools::Itertools;
use std::rc::Rc;
use std::fmt;

pub trait GameLogic {
    fn on_start(obj: &mut Self);
    fn on_update(obj: &mut Self, win: &mut Window, time_elapsed: Rc<f64>);
}

// Handles game logic e.g. timers, physics, etc.
pub struct Game {
    pub tile_atlas: HashMap<Ident, Tile>,
    pub next_tile_id: Ident,
    pub next_actor_id: Ident,
}

impl Game {
    pub fn tile_atlas_contains(&self, pix_buf: &PixBuf) -> bool {
        for tile in self.tile_atlas.values() {
            if tile.pix_buf == *pix_buf { return true; }
        }
        false
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = f.debug_struct("Game")
            .field("next_tile_id", &self.next_tile_id)
            .field("next_sprite_id", &self.next_actor_id)
            .finish();

        for key in self.tile_atlas.keys().sorted() {
            println!();
            if let Some(tile) = self.tile_atlas.get(key) {
                let pix_buf_chars: Vec<char> = tile.pix_buf.iter().map(|&pixel| pixel as char).collect();

                write!(f, "{:<2} [", key)?;
                for (i, c) in pix_buf_chars.iter().enumerate() {
                    write!(f, "{}", c)?;
                    if i < pix_buf_chars.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
            }
        }

        result
    }
}

impl GameLogic for Game {
    fn on_start(obj: &mut Self) {
        println!("Starting game...");
    }

    fn on_update(obj: &mut Self, win: &mut Window, time_elapsed: Rc<f64>) {
        win.render_frame(obj);
    }
}
