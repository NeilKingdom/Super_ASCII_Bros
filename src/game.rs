use crate::ascii_bros::*;
use crate::actor::sprite::tile::Tile;

use std::collections::HashMap;

pub trait GameLogic {
    fn on_start(&self);
    fn on_update(&self);
}

// Handles game logic e.g. timers, physics, etc.
pub struct Game {
    // TODO: Make HashSet
    pub tile_atlas: HashMap<Ident, Tile>,
    pub next_tile_id: Ident,
    pub next_sprite_id: Ident,
    pub next_actor_id: Ident,
    pub fn_on_start: Box<dyn Fn() -> ()>,
    pub fn_on_update: Box<dyn Fn() -> ()>,
}

impl GameLogic for Game {
    fn on_start(&self) {
        (self.fn_on_start)();
    }

    fn on_update(&self) {
        // delta_time = get_delta_time();
        (self.fn_on_update)();
    }
}
