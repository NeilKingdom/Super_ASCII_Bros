#![allow(non_snake_case)]

pub mod ascii_bros;
pub mod game;
pub mod actor;

use crate::game::*;
use crate::actor::Actor;
use crate::ascii_bros::Color;
use crate::actor::sprite::tile::TILE_AREA;

use crate::libc::STDIN_FILENO;
use crate::libc::ioctl;
use crate::actor::sprite::Sprite;

//use signal_hook::{consts::SIGWINCH, iterator::Signals};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::mem::size_of;
use std::thread;
use std::rc::Rc;
use std::io::{self, Read};
use nix::libc;
use std::sync::mpsc::channel;

#[derive(Default)]
struct ShellInfo {
    cols: usize,
    lines: usize,
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

        ShellInfo { cols: winsize.ws_col as usize, lines: winsize.ws_row as usize }
    }
}

#[derive(Default)]
pub struct Window {
    height: usize,          // The height of the window being rendered (this is not necessarily equal to the height of the terminal)
    width: usize,           // The width of the window being rendered (this is not necessarily equal to the width of the terminal)
    color_buf: Vec<Color>,  // Buffer containing color data for each pixel in pix_buf
    pix_buf: String,        // Buffer containing each glyph to be printed when rendering
}

impl Window {
    fn default() -> Self {
        Window {
            height: 25,
            width: 80,
            color_buf: Vec::new(),
            pix_buf: String::new()
        }
    }

    fn clear(&mut self) {
        // TODO: Make constants and place in enum
        print!("\x1b[2J");      // Clear the screen
        print!("\x1b[H");       // Move cursor to position (0, 0)
        print!("\x1b[?25l");    // Hide cursor

        self.color_buf.clear();
        self.pix_buf.clear();
    }

    fn render_frame(&mut self, game: &mut Game) {
        self.clear();

        // Load sprites (TODO: Put into separate function)
        // TODO: Make more idomatic by opening dir entry and reading each file from there using iterator
        let sprite_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/assets/sprites/";
        let ss_mushroom = "mushroom.txt";
        let full_path = OsString::from(sprite_dir + ss_mushroom);
        let mushroom_sprite = Sprite::new(game, PathBuf::from(full_path), 255);

        let mushroom_actor = Actor::new(game, 0.0, 0.0, &mushroom_sprite);

        let screen_area = (self.width + 1) * self.height; // Add 1 to make room for newlines
        // TODO: Error handling
        self.pix_buf.reserve_exact(screen_area);
        dbg!(&self.pix_buf.capacity());

        while self.pix_buf.len() != self.pix_buf.capacity() {
            self.pix_buf.push(' ');
        }

        let sprite_offset = ((mushroom_actor.y_pos.round() * self.width as f32) + mushroom_actor.x_pos.round()) as usize;
        for tile_id in &mushroom_actor.sprite.tile_ids {
            // TODO: This is assuming TILE_AREA = 4. Make to be dynamic
            let mut pixel = game.tile_atlas[&tile_id].pix_buf[0];
            self.pix_buf.insert(sprite_offset, pixel as char);

            pixel = game.tile_atlas[&tile_id].pix_buf[1];
            self.pix_buf.insert(sprite_offset + 1, pixel as char);

            pixel = game.tile_atlas[&tile_id].pix_buf[2];
            self.pix_buf.insert(sprite_offset + self.width, pixel as char);

            pixel = game.tile_atlas[&tile_id].pix_buf[3];
            self.pix_buf.insert(sprite_offset + self.width + 1, pixel as char);
        }

        // Insert newlines at end of screen
        for idx in (self.width..screen_area).step_by(self.width) {
            self.pix_buf.insert(idx, '\n');
        }

        print!("{}", &self.pix_buf);

        /*
        let col_pix_pairs: Vec<(Color, u8)> = ;
        for (color, pixel) in col_pix_pairs.iter() {
            print!("\x1b[0;{}m", color, pixel); // Reset attributes before printing color attribute and pixel glyph
        }
        */
    }
}

struct KeyPress(char);

fn main() -> ! {
    // Get $COLUMNS and $ROWS
    let shell_info: ShellInfo = ShellInfo::default();
    println!("cols: {} lines: {}", shell_info.cols, shell_info.lines);

    // Create a new window
    let mut win = Window::default();
    if win.height > shell_info.lines {
        panic!("Window is too small");
    }

    // Create a game object
    let mut game = Game {
        tile_atlas: HashMap::new(),
        next_tile_id: 0,
        next_actor_id: 0,
    };

    println!("{:#?}", game);

    //let mut signals = Signals::new(&[SIGWINCH])?;
    //thread::spawn(move || {
    //    for _sig in signals.forever() {
    //        todo!();
    //    }
    //});

    let (tx, rx) = channel();

    // Get the current terminal settings
    let orig_termios = Termios::from_fd(0).expect("from_fd failed");

    // Spawn thread for polling keyboard input
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        // Disable canonical mode (line buffering) and echo
        let mut new_termios = orig_termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(0, TCSANOW, &new_termios).expect("tcsetattr failed");

        loop {
            // Read and process input
            let mut input = [0u8; 1];
            while handle.read_exact(&mut input).is_ok() {
                let c = input[0] as char;
                tx.send(c).expect("send failed");
            }
        }
    });

    // Game start
    Game::on_start(&mut game);
    Game::on_update(&mut game, &mut win, Rc::new(0.0));

    // Game loop
    loop {
        // Receive user input from channel
        let c = KeyPress(rx.recv().expect("recv failed"));
        match c {
            KeyPress(' ') => println!("jump"),
            KeyPress('a') => println!("left"),
            KeyPress('d') => println!("right"),
            _ => ()
        }

        if c.0 == 'q' {
            // Exit raw mode
            tcsetattr(0, TCSANOW, &orig_termios).expect("tcsetattr failed");
        }

        //game.on_update();
        // TODO:
        // let sprites = Sprite::load_sprite_sheets() // Don't need IDs - just pass to Actor::new()
        // let actors = Actor::load_actors() // Loads from level data file, IDs are static
        // let render_sprite_batch = sprites.filter(|sprite| { sprite.x > 0 && sprite.x <
        //      win.width ... })
    }
}
