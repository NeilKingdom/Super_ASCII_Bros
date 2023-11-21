pub mod sprite;

use crate::Sprite;

// Basic behaviors shared by all actors
pub trait ActorActions {
    fn update_pos(&self, actor_props: &mut ActorProps, delta_time: &u128);
}

// Basic properties shared by all actors
pub struct ActorProps {
    pub x_pos: f32,             // The x position of the actor in sub-pixels
    pub y_pos: f32,             // The y position of the actor in sub-pixels
    pub sprite: Sprite,         // A read-only reference to the actor's associated sprite
}
    
impl ActorProps {
    pub fn new(x_pos: f32, y_pos: f32, sprite: Sprite) -> Self {
        ActorProps { x_pos, y_pos, sprite }
    }
}
 
// Actions for each type of actor
pub struct MushroomActions;
impl ActorActions for MushroomActions {
    fn update_pos(&self, actor_props: &mut ActorProps, delta_time: &u128) {
        actor_props.x_pos += 0.1; // TODO: * delta_time;
    }
}

// The actual actor type
pub struct Actor {
    pub props: ActorProps,
    pub actions: Box<dyn ActorActions>,
}

impl Actor {
    pub fn new(props: ActorProps, actions: Box<dyn ActorActions>) -> Self {
        Actor { props, actions }
    }
}
