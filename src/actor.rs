pub mod sprite;

use crate::{Window, Sprite};
use crate::game::*;
use crate::ascii_bros::*;

use std::rc::Rc;

enum ActorType {
    Mario,
    Bowser,
    Goomba,
    Koopa,
    Parakoopa,
    Pirhana,
    Lakitu,
    Spiney,
    Beetle,
    BulletBill,
    HammerBro,
    FireBar,
}

pub struct Actor<'a> {
    pub id: Ident,
    pub x_pos: f32,
    pub y_pos: f32,
    pub sprite: &'a Sprite,
}

impl<'a> Actor<'a> {
    pub fn new(
        game: &Game,
        x_pos: f32,
        y_pos: f32,
        sprite: &'a Sprite,
    ) -> Self {
        Actor {
            id: game.next_actor_id,
            x_pos,
            y_pos,
            sprite,
        }
    }
}

impl<'a> GameLogic for Actor<'a> {
    fn on_start(obj: &mut Self) {
        todo!();
    }

    fn on_update(obj: &mut Self, win: &mut Window, time_elapsed: Rc<f64>) {
        todo!();
    }
}
