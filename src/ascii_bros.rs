pub type Ident = u16;

#[derive(Default, Debug, Copy, Clone)]
pub struct Color(pub i32);

pub const TARGET_FPS: u8 = 30;

// Binds actors to their respective sprites
#[derive(Debug, Default)]
pub enum EntityType {
    #[default]
    None,
    Mario,
    Mushroom,
    OneUp,
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

