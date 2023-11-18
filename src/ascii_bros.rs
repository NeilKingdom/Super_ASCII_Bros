pub type Ident = u16;

#[derive(Default, Debug, Copy, Clone)]
pub struct Color(pub i32);

pub struct KeyPress(pub char);

pub const SCRN_CLR: &str    = "\x1b[2J";
pub const CUR_POS_RST: &str = "\x1b[H";
pub const CUR_HIDE: &str    = "\x1b[?251";
pub const LN_WRP_OFF: &str  = "\x1b[?71";

pub const SECOND_IN_MILLIS: f32 = 1000.0;
pub const TARGET_FPS: f32 = 1.0;

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

