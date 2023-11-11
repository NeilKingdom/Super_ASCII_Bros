pub mod sprite;

use crate::Sprite;
use crate::ascii_bros::*;

use std::error::Error;

type ActorFnPtr<T> = Box<dyn FnMut(&mut Actor<T>, &[T]) -> Result<(), Box<dyn Error>>>;

pub struct Actor<T> {
    pub r#type: EntityType,     // The type of actor. Multiple actors may share the same type.
    pub x_pos: f32,             // The x position of the actor in sub-pixels
    pub y_pos: f32,             // The y position of the actor in sub-pixels
    pub sprite: Sprite,         // A read-only reference to the actor's associated sprite
    pub on_update: Option<ActorFnPtr<T>>,
}

impl<T> Actor<T> {
    pub fn new(
        r#type: EntityType,
        x_pos: f32,
        y_pos: f32,
        sprite: Sprite,
        on_update: Option<ActorFnPtr<T>>,
    ) -> Self {
        Actor {
            r#type,
            x_pos,
            y_pos,
            sprite,
            on_update
        }
    }

    pub fn call_on_update(&mut self, args: &[T]) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut on_update) = self.on_update {
            on_update(self, args)
        } else {
            Ok(())
        }
    }
}
