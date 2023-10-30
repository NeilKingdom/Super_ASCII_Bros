use crate::ascii_bros::*;
use crate::actor::sprite::tile::Tile;

use std::collections::HashMap;
use itertools::Itertools;
use std::rc::Rc;
use std::fmt;

pub trait GameLogic {
    fn on_start(&self);
    fn on_update(&self, timeElapsed: Rc<f64>);
}

// Handles game logic e.g. timers, physics, etc.
pub struct Game {
    // TODO: Make HashSet
    pub tile_atlas: HashMap<Ident, Tile>,
    pub next_tile_id: Ident,
    pub next_sprite_id: Ident,
    pub fn_on_start: Box<dyn Fn() -> ()>,
    pub fn_on_update: Box<dyn Fn(Rc<f64>) -> ()>,
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = f.debug_struct("Game")
            .field("next_tile_id", &self.next_tile_id)
            .field("next_sprite_id", &self.next_sprite_id)
            .finish();


        for key in self.tile_atlas.keys().sorted() {
            println!();
            write!(f, "{:<2} [", key)?;
            let tile = self.tile_atlas.get(key);
            for pixel in tile.unwrap().pix_buf {
                write!(f, "{}, ", pixel as char)?;
            }
            write!(f, "]")?;
        }

        result
    }
}

impl GameLogic for Game {
    fn on_start(&self) {
        (self.fn_on_start)();
    }

    fn on_update(&self, timeElapsed: Rc<f64>) {
        (self.fn_on_update)(timeElapsed);
    }
}
