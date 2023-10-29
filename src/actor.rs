pub mod sprite;

use crate::Sprite;
use crate::game::*;
use crate::ascii_bros::*;

struct Actor {
    id: Ident,
    sprite: Sprite,
    fn_on_start: Box<dyn Fn() -> ()>,
    fn_on_update: Box<dyn Fn() -> ()>,
}

impl GameLogic for Actor {
    fn on_start(&self) {
        (self.fn_on_start)();
    }

    fn on_update(&self) {
        (self.fn_on_update)();
    }
}

