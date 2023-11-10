pub mod sprite;

use crate::Sprite;
use crate::game::*;
use crate::ascii_bros::*;

pub struct Actor<'a> {
    pub r#type: EntityType,     // The type of actor. Multiple actors may share the same type.
    pub x_pos: f32,             // The x position of the actor in sub-pixels
    pub y_pos: f32,             // The y position of the actor in sub-pixels
    pub sprite: &'a Sprite,     // A read-only reference to the actor's associated sprite
}

impl<'a> Actor<'a> {
    pub fn new(
        game: &Game,
        r#type: EntityType,
        x_pos: f32,
        y_pos: f32,
        sprite: &'a Sprite,
    ) -> Self {
        Actor {
            r#type,
            x_pos,
            y_pos,
            sprite,
        }
    }
}
