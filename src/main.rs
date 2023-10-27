#![allow(non_snake_case)]

use crate::libc::STDIN_FILENO;
use crate::libc::ioctl;
use nix::libc;

//use signal_hook::{consts::SIGWINCH, iterator::Signals};
use std::collections::HashMap;
use std::ffi::OsString;
use std::error::Error;
use std::path::PathBuf;
use std::thread;
use std::fmt;
use std::fs;
use itertools::Itertools;

#[derive(Default)]
struct ShellInfo {
    cols: u16,
    lines: u16,
}

impl ShellInfo {
    fn default() -> Self {

        // Initialize a winsize struct to store the terminal's column & line count
        let mut winsize = libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        // Use the TIOCGWINSZ ioctl command to query the terminal dimensions
        if unsafe { ioctl(STDIN_FILENO, libc::TIOCGWINSZ, &mut winsize) } == -1 {
            panic!("Failed to retrieve terminal size");
        }

        ShellInfo { cols: winsize.ws_col, lines: winsize.ws_row }
    }
}

#[derive(Default, Debug, Copy, Clone)]
enum Color {
    #[default]
    None,
    FgColor(u8),
    BgColor(u8),
}

#[derive(Default)]
struct Window {
    height: u16,            // The height of the window being rendered (this is not necessarily equal to the height of the terminal)
    width: u16,             // The width of the window being rendered (this is not necessarily equal to the width of the terminal)
    stride: usize,          // The stride of both color_buf and pix_buf
    color_buf: Vec<Color>,  // Buffer containing color data for each pixel in pix_buf
    pix_buf: OsString,      // Buffer containing each glyph to be printed when rendering
}

impl Window {
    fn default() -> Self {
        Window {
            height: 25,
            width: 80,
            stride: 0,
            color_buf: Vec::new(),
            pix_buf: OsString::new() }
    }

    fn clear() {
        print!("\x1b[2J");      // Clear the screen
        print!("\x1b[H");       // Move cursor to position (0, 0)
        print!("\x1b[?25l");    // Hide cursor
    }

    // TODO: Take in tile_atlas, actors, window
    fn render_frame(&self) {
        Self::clear();

        /*
        let col_pix_pairs: Vec<(Color, u8)> = ;
        for (color, pixel) in col_pix_pairs.iter() {
            print!("\x1b[0;{}m", color, pixel); // Reset attributes before printing color attribute and pixel glyph
        }
        */
    }
}

trait StringExtensions {
    fn as_tile_ids(&self, tile_atlas: &mut HashMap<Ident, Tile>) -> Vec<Ident>;
}

// TODO: Add tests for this function
impl StringExtensions for String {
    fn as_tile_ids(&self, tile_atlas: &mut HashMap<Ident, Tile>) -> Vec<Ident> {
        let lines: Vec<&str> = self.split("\n").collect();
        let len = self.replace("\n", "").len();
        debug_assert!(len == 256);

        let mut pix_bufs: Vec<Tile> = vec![Tile::default(); len / TILE_AREA];
        let mut tile_ids: Vec<Ident> = Vec::new();

        let (mut prev_pixel_idx, mut pixel_idx, mut tile_idx): (usize, usize, usize);
        let stride = (TILE_AREA as f64).sqrt() as usize;

        for y in 0..(lines.len() - 1) {
            prev_pixel_idx = (y * stride) % TILE_AREA;
            pixel_idx = prev_pixel_idx;

            for x in 0..(lines.len() - 1) {
                tile_idx = ((y / stride) * (lines.len() / stride)) + (x / stride);
                pix_bufs[tile_idx].pix_buf[pixel_idx] = lines[y].as_bytes()[x];

                if pixel_idx == (TILE_AREA - 1) {
                    unsafe { // Unsafe due to incrementing static variable NEXT_TILE_ID
                        let tile = Tile::new(
                            NEXT_TILE_ID,
                            stride,
                            // TODO: Implement
                            [ Color::FgColor(32); TILE_AREA ],
                            pix_bufs[tile_idx].pix_buf
                        );

                        tile_atlas.insert(NEXT_TILE_ID, tile);
                        tile_ids.push(NEXT_TILE_ID);
                        NEXT_TILE_ID += 1;
                    }
                }

                pixel_idx += 1;
                if pixel_idx > (((y % stride) + 1) * stride) - 1 {
                    pixel_idx = prev_pixel_idx;
                }
            }
            println!();
        }

        tile_ids
    }
}

type Ident = u16;
static mut NEXT_TILE_ID: Ident = 0;
static mut NEXT_SPRITE_ID: Ident = 0;
const TILE_AREA: usize = 4;

#[derive(Default, Debug, Clone, Copy)]
struct Tile {
    id: Ident,                      // The tile's unique identifier
    stride: usize,                  // Stride for both color_buf and pix_buf
    // TODO: Do i make this a tuple for fg and bg colors?
    color_buf: [Color; TILE_AREA],  // The color data corresponding to each pixel in the tile
    pix_buf: [u8; TILE_AREA],       // The "pixel" data i.e. sequence of characters
}

impl Tile {
    fn new(
        id: Ident,
        stride: usize,
        color_buf: [Color; TILE_AREA],
        pix_buf: [u8; TILE_AREA]
    ) -> Self {
        Tile { id, stride, color_buf, pix_buf }
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("id", &self.id)
            .field("stride", &self.stride)
            .field("color_buf", &self.color_buf)
            .field("pix_buf", &self.pix_buf)
            .finish()
    }
}

#[derive(Default, Debug)]
struct Sprite {
    id: Ident,                  // The sprite's unique identifier
    file_handle: PathBuf,       // The path at which to find the sprite data
    tile_data: Vec<Ident>,      // A vector of Tile IDs ordered from left to right, top to bottom
    x_pos: f32,                 // The current x position of the sprite (in sub-pixels)
    y_pos: f32,                 // The current y position of the sprite (in sub-pixels)
    z_order: u8                 // The z layer priority of the sprite when rendered
}

impl Sprite {
    fn default() -> Self {
        Sprite {
            // TODO: Increment
            id: unsafe { NEXT_SPRITE_ID },
            file_handle: PathBuf::new(),
            tile_data: Vec::new(),
            x_pos: 0.0,
            y_pos: 0.0,
            z_order: 0
        }
    }

    fn validate(&self) -> Result<(), String> {
        // Sprite tile data size should be a perfect square and multiple of TILE_AREA
        if (self.tile_data.len() as f64).sqrt() % (TILE_AREA as f64) != 0.0 {
            Result::Err(format!("Sprite {}'s tile data is of an invalid size", self.id))
        } else {
            Result::Ok(())
        }
    }

    fn load_raster_from_file(file_handle: &PathBuf) -> Result<String, Box<dyn Error>> {
        let pix_buf = fs::read_to_string(file_handle.as_path())?.replace("@", " ");
        println!("{}", pix_buf);
        Ok(pix_buf)
     }

    fn new(tile_atlas: &mut HashMap<Ident, Tile>, file_handle: PathBuf, z_order: u8) -> Self {
        let pix_buf = Self::load_raster_from_file(&file_handle)
            .expect("Failed to load sprite data from file");

        let mut sprite: Sprite = Sprite::default();
        sprite.file_handle = file_handle;
        sprite.tile_data = pix_buf.as_tile_ids(tile_atlas);
        sprite.z_order = z_order;

        let result = sprite.validate();
        match result {
            Result::Err(message) => { panic!("{}", message); },
            _ => ()
        }

        sprite
    }
}

trait GameLogic {
    fn on_start(&self);
    fn on_update(&self);
}

// Handles game logic e.g. timers, physics, etc.
struct Game {
    // TODO: Make HashSet
    tile_atlas: HashMap<Ident, Tile>,
    fn_on_start: Box<dyn Fn() -> ()>,
    fn_on_update: Box<dyn Fn() -> ()>,
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

fn main() -> Result<(), Box<dyn Error>> {
    // Get $COLUMNS and $ROWS
    let shell_info: ShellInfo = ShellInfo::default();
    println!("cols: {} lines: {}", shell_info.cols, shell_info.lines);

    // Create a new window
    let win = Window::default();
    if win.height > shell_info.lines {
        panic!("Window is too small");
    }

    // Create a game entity
    let mut game = Game {
        tile_atlas: HashMap::new(),
        fn_on_start: Box::new(|| {
            println!("Game starting...");
        }),
        fn_on_update: Box::new(move || {
            win.render_frame();
        })
    };

    // Load sprites (TODO: Put into separate function)
    let mushroom_sprite = Sprite::new(&mut game.tile_atlas, PathBuf::from("assets/sprites/mushroom.txt"), 255);

    // TODO: Implement as Debug trait for tile_atlas
    for ident in game.tile_atlas.keys().sorted() {
        print!("{:<2} [", ident);
        let tile = game.tile_atlas.get(ident);
        for pixel in tile.unwrap().pix_buf {
            print!("{}, ", pixel as char);
        }
        println!("]");
    }

    /*
    let mut signals = Signals::new(&[SIGWINCH])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            todo!();
        }
    });
    */

    // Spawn thread for polling keyboard input
    thread::spawn(|| {
        loop {
            // TODO: Set terminal to raw mode using ansi control code

            /*match usr_input.as_str() {
                " " => { println!("Jump") },
                "a" => { println!("Left") },
                "d" => { println!("Right") },
                _ => ()
            }
            */
        }
    });

    // Game start
    game.on_start();

    // Game loop
    loop {
        //game.on_update();
        // TODO:
        // for actor in actors {
        //    actor.on_update();
        // }
    }
}
